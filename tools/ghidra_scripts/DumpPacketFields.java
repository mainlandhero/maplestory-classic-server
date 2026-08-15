// Label outbound opcodes by the shape of the packet each builder writes.
//
//   -postScript DumpPacketFields.java <out-file> <writer-begin-addr>
//
// FUN_1406ed520(buf, opcode) starts a packet; the calls that follow in the same function
// append its fields. Recording that call sequence gives every opcode a structural
// signature - "u8,u32,u32,u8" - which is far more useful than the bare number, and any
// string the builder references usually names it outright.
//
// This is the only reliable way to label opcodes for this client: the *inbound*
// dispatcher lives in the .themida section and has no file bytes, so inbound opcodes
// cannot be recovered this way at all. Outbound is what we can actually read.
//
//@category MapleCW

import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Data;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.scalar.Scalar;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.ReferenceIterator;

public class DumpPacketFields extends GhidraScript {

    /** Writer helpers already identified by hand; the rest are reported raw. */
    private static final Map<String, String> KNOWN = new LinkedHashMap<>();
    static {
        KNOWN.put("FUN_1406ed840", "u8");
        KNOWN.put("FUN_1406ed9d0", "u32");
    }

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args == null || args.length < 2) {
            println("usage: DumpPacketFields <out-file> <writer-begin-addr>");
            return;
        }
        PrintWriter out = new PrintWriter(args[0], "UTF-8");
        Address begin = currentProgram.getAddressFactory().getAddress(args[1].replace("0x", ""));

        // Count how often each helper appears, so the unnamed ones can be identified by
        // frequency and by the company they keep.
        Map<String, Integer> helperUse = new LinkedHashMap<>();
        List<String> rows = new ArrayList<>();

        ReferenceIterator refs = currentProgram.getReferenceManager().getReferencesTo(begin);
        while (refs.hasNext() && !monitor.isCancelled()) {
            Address from = refs.next().getFromAddress();
            Function f = getFunctionContaining(from);
            if (f == null) {
                continue;
            }

            Long opcode = immediateBefore(from, "EDX");
            List<String> fields = new ArrayList<>();
            List<String> strings = new ArrayList<>();

            Instruction ins = getInstructionAt(from);
            while (ins != null && f.getBody().contains(ins.getAddress())) {
                if (ins.getMnemonicString().equalsIgnoreCase("CALL")) {
                    Address[] flows = ins.getFlows();
                    if (flows.length > 0 && !flows[0].equals(begin)) {
                        Function callee = getFunctionAt(flows[0]);
                        if (callee != null) {
                            String n = callee.getName();
                            fields.add(KNOWN.getOrDefault(n, n));
                            helperUse.merge(n, 1, Integer::sum);
                        }
                    }
                }
                for (Reference r : ins.getReferencesFrom()) {
                    Data d = getDataAt(r.getToAddress());
                    if (d != null && d.hasStringValue()) {
                        String v = d.getValue().toString().trim();
                        if (!v.isEmpty() && v.length() < 60) {
                            strings.add(v);
                        }
                    }
                }
                ins = ins.getNext();
            }

            String op = opcode == null ? "????" : String.format("0x%04X", opcode);
            rows.add(String.format("%s  %-22s  %s%s", op, f.getName(),
                    String.join(",", fields.size() > 24 ? fields.subList(0, 24) : fields),
                    strings.isEmpty() ? "" : "   // " + String.join(" | ", strings)));
        }

        rows.sort(String::compareTo);
        out.println("# outbound opcodes with the field sequence each builder writes");
        out.println("# " + rows.size() + " call sites");
        out.println();
        for (String r : rows) {
            out.println(r);
        }

        out.println();
        out.println("# helper call frequency - use this to name the unidentified writers");
        helperUse.entrySet().stream()
                .sorted((a, b) -> b.getValue() - a.getValue())
                .limit(40)
                .forEach(e -> out.println(String.format("  %-22s %d", e.getKey(), e.getValue())));
        out.close();
        println("wrote " + args[0] + " (" + rows.size() + " rows)");
    }

    private Long immediateBefore(Address call, String reg) {
        Instruction ins = getInstructionAt(call);
        for (int i = 0; i < 16 && ins != null; i++) {
            ins = ins.getPrevious();
            if (ins == null) {
                break;
            }
            String m = ins.getMnemonicString();
            if (!m.equalsIgnoreCase("MOV") && !m.equalsIgnoreCase("XOR")) {
                continue;
            }
            Object[] dst = ins.getOpObjects(0);
            if (dst.length == 0 || !dst[0].toString().equalsIgnoreCase(reg)) {
                continue;
            }
            if (m.equalsIgnoreCase("XOR")) {
                Object[] src = ins.getOpObjects(1);
                return (src.length > 0 && src[0].toString().equalsIgnoreCase(reg)) ? 0L : null;
            }
            for (Object o : ins.getOpObjects(1)) {
                if (o instanceof Scalar) {
                    return ((Scalar) o).getUnsignedValue();
                }
            }
            return null;
        }
        return null;
    }
}
