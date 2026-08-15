// Dump every opcode the client builds, by inspecting calls to the packet writer.
//
//   -postScript DumpOpcodes.java <out-file> <writer-hex-addr> [arg-index]
//
// FUN_1406ed520(buf, opcode) begins an outbound packet. Every call site therefore names
// one client -> server opcode, and the calling function is its builder. Enumerating them
// gives the outbound half of the protocol without needing to decrypt anything - which
// matters here, because the send path itself is Themida-obfuscated.
//
// Win64 fastcall puts the second integer argument in EDX/RDX, so this walks backwards
// from each call looking for the immediate that lands there.
//
//@category MapleCW

import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.List;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.scalar.Scalar;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.ReferenceIterator;

public class DumpOpcodes extends GhidraScript {

    private static final int WINDOW = 16;

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args == null || args.length < 2) {
            println("usage: DumpOpcodes <out-file> <writer-hex-addr> [reg]");
            return;
        }

        PrintWriter out = new PrintWriter(args[0], "UTF-8");
        Address target = currentProgram.getAddressFactory().getAddress(args[1].replace("0x", ""));
        String reg = args.length > 2 ? args[2].toUpperCase() : "EDX";

        ReferenceIterator refs = currentProgram.getReferenceManager().getReferencesTo(target);
        List<String> rows = new ArrayList<>();
        int total = 0, resolved = 0;

        while (refs.hasNext() && !monitor.isCancelled()) {
            Reference r = refs.next();
            Address from = r.getFromAddress();
            Function caller = getFunctionContaining(from);
            total++;

            // Walk back looking for the immediate that sets the opcode register.
            Long opcode = null;
            Instruction ins = getInstructionAt(from);
            for (int i = 0; i < WINDOW && ins != null; i++) {
                ins = ins.getPrevious();
                if (ins == null) {
                    break;
                }
                String mnem = ins.getMnemonicString();
                if (!mnem.equalsIgnoreCase("MOV") && !mnem.equalsIgnoreCase("XOR")
                        && !mnem.equalsIgnoreCase("LEA")) {
                    continue;
                }
                Object[] dst = ins.getOpObjects(0);
                if (dst.length == 0 || !dst[0].toString().toUpperCase().equals(reg)) {
                    continue;
                }
                if (mnem.equalsIgnoreCase("XOR")) {
                    // `xor edx, edx` is opcode 0.
                    Object[] src = ins.getOpObjects(1);
                    if (src.length > 0 && src[0].toString().toUpperCase().equals(reg)) {
                        opcode = 0L;
                    }
                    break;
                }
                for (Object o : ins.getOpObjects(1)) {
                    if (o instanceof Scalar) {
                        opcode = ((Scalar) o).getUnsignedValue();
                    }
                }
                break;
            }

            String name = caller == null ? "<no function>"
                    : caller.getName() + " @ " + caller.getEntryPoint();
            if (opcode != null) {
                resolved++;
                rows.add(String.format("0x%04X  %-5d  %s  (call at %s)", opcode, opcode, name, from));
            } else {
                rows.add(String.format("  ????         %s  (call at %s)", name, from));
            }
        }

        rows.sort(String::compareTo);
        out.println("# opcodes passed to " + target + " in " + reg);
        out.println("# " + resolved + " resolved of " + total + " call sites");
        out.println();
        for (String row : rows) {
            out.println(row);
        }
        out.close();
        println("wrote " + args[0] + " (" + resolved + "/" + total + ")");
    }
}
