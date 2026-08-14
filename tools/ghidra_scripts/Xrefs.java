// List the functions that reference the given addresses (i.e. callers).
//
//   -postScript Xrefs.java 142c94bd0 [more addresses...]
//
// DecompileFunc walks *downward* into callees; this walks upward, which is how the
// consumers of a parsed config struct get found.
//
//@category MapleCW

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.ReferenceIterator;

public class Xrefs extends GhidraScript {

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args == null || args.length == 0) {
            println("usage: Xrefs <hex-address> [...]");
            return;
        }

        for (String a : args) {
            Address addr = currentProgram.getAddressFactory().getAddress(a.replace("0x", ""));
            if (addr == null) {
                println("bad address: " + a);
                continue;
            }

            Function target = getFunctionContaining(addr);
            println("=== references to " + addr
                    + (target == null ? "" : "  (" + target.getName() + ")"));

            ReferenceIterator refs = currentProgram.getReferenceManager().getReferencesTo(addr);
            int n = 0;
            while (refs.hasNext() && n < 60) {
                Reference r = refs.next();
                Address from = r.getFromAddress();
                Function f = getFunctionContaining(from);
                String where = (f == null)
                        ? "<no function>"
                        : f.getName() + " @ " + f.getEntryPoint()
                          + " (" + f.getBody().getNumAddresses() + " bytes)";
                println("    " + from + "  " + r.getReferenceType() + "  in  " + where);
                n++;
            }
            if (n == 0) {
                println("    (none)");
            }
            println("");
        }
    }
}
