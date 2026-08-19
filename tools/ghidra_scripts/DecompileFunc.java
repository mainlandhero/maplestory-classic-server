// Decompile functions by address.
//
//   -postScript DecompileFunc.java <out-file> 142c94bd0 [more addresses...] [+callees]
//   -postScript DecompileFunc.java <out-file> @addresses.txt
//
// "@file" reads one hex address per line - cmd.exe truncates a command line at 8191
// characters, which is fewer than a few hundred addresses.
//
// Output goes straight to <out-file>: Ghidra's logger flattens multi-line println
// output, which mangles decompiled C when scraped from stdout.
//
// Optionally follows callees one level deep with the "+callees" flag, which is how the
// argument parser's helpers get pulled in.
//
//@category MapleCW

import java.io.PrintWriter;
import java.util.LinkedHashSet;
import java.util.Set;

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;

public class DecompileFunc extends GhidraScript {

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args == null || args.length < 2) {
            println("usage: DecompileFunc <out-file> <hex-address|@listfile> [...] [+callees]");
            return;
        }

        PrintWriter out = new PrintWriter(args[0], "UTF-8");
        boolean withCallees = false;
        Set<Address> targets = new LinkedHashSet<>();
        java.util.List<String> words = new java.util.ArrayList<>();
        for (int i = 1; i < args.length; i++) {
            // "@file" reads one hex address per line. cmd.exe truncates a command line at
            // 8191 characters, so a sweep of several hundred handlers cannot be passed
            // as arguments at all.
            if (args[i].startsWith("@")) {
                for (String line : java.nio.file.Files.readAllLines(
                        java.nio.file.Paths.get(args[i].substring(1)))) {
                    String t = line.trim();
                    if (!t.isEmpty() && !t.startsWith("#")) {
                        words.add(t);
                    }
                }
            } else {
                words.add(args[i]);
            }
        }
        for (String a : words) {
            if (a.equalsIgnoreCase("+callees")) {
                withCallees = true;
                continue;
            }
            Address addr = currentProgram.getAddressFactory().getAddress(a.replace("0x", ""));
            if (addr == null) {
                println("bad address: " + a);
                continue;
            }
            targets.add(addr);
        }

        DecompInterface d = new DecompInterface();
        d.openProgram(currentProgram);

        Set<Address> done = new LinkedHashSet<>();
        Set<Address> queue = new LinkedHashSet<>(targets);

        while (!queue.isEmpty() && !monitor.isCancelled()) {
            Address addr = queue.iterator().next();
            queue.remove(addr);
            if (!done.add(addr)) {
                continue;
            }

            Function f = getFunctionContaining(addr);
            if (f == null) {
                // Auto-analysis only creates functions it can reach, and code reached only
                // from the Themida VM has no caller in .text - so the interesting handlers
                // are exactly the ones missing. .pdata still bounds them, so create one.
                println("no function at " + addr + "; creating one");
                f = createFunction(addr, null);
            }
            if (f == null) {
                out.println("### no function at " + addr + ", and one could not be created");
                continue;
            }

            println("decompiling " + f.getName() + " @ " + f.getEntryPoint());
            out.println("");
            out.println("//===========================================================");
            out.println("// " + f.getName() + " @ " + f.getEntryPoint()
                    + "   (" + f.getBody().getNumAddresses() + " bytes)");
            out.println("//===========================================================");

            DecompileResults res = d.decompileFunction(f, 180, monitor);
            if (res != null && res.decompileCompleted()) {
                out.println(res.getDecompiledFunction().getC());
            } else {
                out.println("// decompilation failed: "
                        + (res == null ? "no result" : res.getErrorMessage()));
            }
            out.flush();

            if (withCallees && done.size() < 12) {
                for (Function callee : f.getCalledFunctions(monitor)) {
                    if (!done.contains(callee.getEntryPoint())) {
                        queue.add(callee.getEntryPoint());
                    }
                }
            }
        }
        d.dispose();
        out.close();
        println("wrote " + args[0]);
    }
}
