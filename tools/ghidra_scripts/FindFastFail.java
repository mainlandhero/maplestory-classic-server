// Find the real `int 0x29` (__fastfail) sites, and say what reaches them.
//
// The client exits 0xC0000409 on a fixed ~36.9s deadline from launch, with no patches of
// ours in the image. That status is what `int 0x29` produces, so the question is which of
// these sites runs and what decides it.
//
// A raw byte scan for CD 29 is not instruction-aligned and over-reports badly. This walks
// the disassembly instead, so every hit is an instruction Ghidra actually decoded, and it
// records the instructions immediately before each one - the fail-fast reason code is
// loaded into ECX right there, and that code says whether it is a stack cookie check, a
// CFG failure, or a deliberate "stop now".
//
// Usage: -postScript FindFastFail.java <outfile>
//@category MapleCW
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.CodeUnit;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.InstructionIterator;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.symbol.Reference;

import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.List;

public class FindFastFail extends GhidraScript {

    private static final int CONTEXT = 6;

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 1) {
            println("FindFastFail: need an output file");
            return;
        }
        PrintWriter out = new PrintWriter(args[0], "UTF-8");
        try {
            Listing listing = currentProgram.getListing();
            InstructionIterator it = listing.getInstructions(true);
            int scanned = 0;
            List<Instruction> hits = new ArrayList<>();
            while (it.hasNext() && !monitor.isCancelled()) {
                Instruction ins = it.next();
                scanned++;
                if (!ins.getMnemonicString().equalsIgnoreCase("INT")) {
                    continue;
                }
                String op = ins.getDefaultOperandRepresentation(0);
                if (op == null) {
                    continue;
                }
                op = op.trim().toLowerCase();
                if (op.equals("0x29") || op.equals("29h") || op.equals("41")) {
                    hits.add(ins);
                }
            }
            out.printf("scanned %d instructions, found %d real `int 0x29` sites%n%n", scanned, hits.size());

            for (Instruction ins : hits) {
                Address at = ins.getAddress();
                MemoryBlock block = currentProgram.getMemory().getBlock(at);
                Function f = getFunctionContaining(at);
                out.printf("=== %s   section=%s   function=%s%n", at,
                        block == null ? "?" : block.getName(),
                        f == null ? "(none)" : f.getName() + " @ " + f.getEntryPoint());

                // Walk backwards for context. The reason code is in ECX at this point.
                List<String> before = new ArrayList<>();
                Instruction cur = ins.getPrevious();
                for (int i = 0; i < CONTEXT && cur != null; i++) {
                    before.add(0, String.format("    %s  %s", cur.getAddress(), cur.toString()));
                    cur = cur.getPrevious();
                }
                for (String line : before) {
                    out.println(line);
                }
                out.printf("  > %s  %s%n", at, ins.toString());

                if (f != null) {
                    int callers = 0;
                    StringBuilder who = new StringBuilder();
                    for (Reference r : getReferencesTo(f.getEntryPoint())) {
                        if (r.getReferenceType().isCall()) {
                            callers++;
                            if (callers <= 8) {
                                who.append(' ').append(r.getFromAddress());
                            }
                        }
                    }
                    out.printf("    callers of %s: %d%s%s%n", f.getName(), callers, who,
                            callers > 8 ? " ..." : "");
                }
                CodeUnit cu = listing.getCodeUnitAt(at);
                if (cu != null && cu.getComment(CodeUnit.PRE_COMMENT) != null) {
                    out.printf("    comment: %s%n", cu.getComment(CodeUnit.PRE_COMMENT));
                }
                out.println();
            }
        } finally {
            out.close();
        }
        println("FindFastFail: wrote " + args[0]);
    }
}
