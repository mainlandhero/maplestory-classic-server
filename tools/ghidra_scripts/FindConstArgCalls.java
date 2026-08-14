// Find calls to a function that pass a given constant, and report the calling function.
//
//   -postScript FindConstArgCalls.java <out-file> <target-hex-addr> <constant-decimal>
//
// Used to answer "who asks for string ID 106?" — the client resolves user-facing
// messages by integer ID, so the caller that passes a particular ID is the code that
// produces that message.
//
// Scans each caller's instructions in a short window before the call for an immediate
// operand matching the constant, which is how a small ID reaches the argument register.
//
//@category MapleCW

import java.io.PrintWriter;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.scalar.Scalar;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.ReferenceIterator;

public class FindConstArgCalls extends GhidraScript {

    /** How many instructions before the call to search for the constant. */
    private static final int WINDOW = 12;

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args == null || args.length < 3) {
            println("usage: FindConstArgCalls <out-file> <target-hex-addr> <constant-decimal>");
            return;
        }

        PrintWriter out = new PrintWriter(args[0], "UTF-8");
        Address target = currentProgram.getAddressFactory().getAddress(args[1].replace("0x", ""));
        long wanted = Long.parseLong(args[2]);

        Function tf = getFunctionContaining(target);
        out.println("# calls to " + target + (tf == null ? "" : " (" + tf.getName() + ")")
                + " passing constant " + wanted + " (0x" + Long.toHexString(wanted) + ")");

        ReferenceIterator refs = currentProgram.getReferenceManager().getReferencesTo(target);
        int examined = 0;
        int hits = 0;

        while (refs.hasNext() && !monitor.isCancelled()) {
            Reference r = refs.next();
            Address from = r.getFromAddress();
            examined++;

            Instruction call = getInstructionAt(from);
            if (call == null) {
                continue;
            }

            // Walk backwards looking for the constant as an immediate operand.
            Instruction ins = call;
            boolean found = false;
            String where = "";
            for (int i = 0; i < WINDOW && ins != null; i++) {
                ins = ins.getPrevious();
                if (ins == null) {
                    break;
                }
                for (int op = 0; op < ins.getNumOperands(); op++) {
                    for (Object o : ins.getOpObjects(op)) {
                        if (o instanceof Scalar && ((Scalar) o).getUnsignedValue() == wanted) {
                            found = true;
                            where = ins.getAddress() + "  " + ins;
                        }
                    }
                }
                if (found) {
                    break;
                }
            }

            if (found) {
                hits++;
                Function f = getFunctionContaining(from);
                out.println("");
                out.println("HIT  call at " + from);
                out.println("     in function " + (f == null ? "<none>"
                        : f.getName() + " @ " + f.getEntryPoint()));
                out.println("     constant set by: " + where);
            }
        }

        out.println("");
        out.println("# examined " + examined + " call sites, " + hits + " matched");
        out.close();
        println("wrote " + args[0] + " (" + hits + " hits of " + examined + " call sites)");
    }
}
