// Decompile a program's exported functions.
//
// Used to learn what the real grap64.dll entry points do and — critically — what
// they return on success, so crates/grap-stub can return the same thing.
//
//@category MapleCW

import java.util.ArrayList;
import java.util.List;

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolIterator;
import ghidra.program.model.symbol.SymbolTable;

public class DumpExports extends GhidraScript {

    @Override
    public void run() throws Exception {
        SymbolTable st = currentProgram.getSymbolTable();
        List<Symbol> exports = new ArrayList<>();

        SymbolIterator it = st.getAllSymbols(true);
        while (it.hasNext()) {
            Symbol s = it.next();
            if (s.isExternalEntryPoint()) {
                exports.add(s);
            }
        }

        println("==== " + currentProgram.getName() + ": " + exports.size() + " exported entry points ====");

        DecompInterface d = new DecompInterface();
        d.openProgram(currentProgram);

        for (Symbol s : exports) {
            Address a = s.getAddress();
            Function f = getFunctionAt(a);
            println("");
            println("---------------------------------------------------------------");
            println("EXPORT " + s.getName() + " @ " + a);
            if (f == null) {
                println("  (no function defined here)");
                continue;
            }
            println("  signature: " + f.getSignature().getPrototypeString());
            println("  size     : " + f.getBody().getNumAddresses() + " bytes");

            DecompileResults res = d.decompileFunction(f, 90, monitor);
            if (res != null && res.decompileCompleted()) {
                println(res.getDecompiledFunction().getC());
            } else {
                println("  (decompilation failed: "
                        + (res == null ? "no result" : res.getErrorMessage()) + ")");
            }
        }
        d.dispose();
    }
}
