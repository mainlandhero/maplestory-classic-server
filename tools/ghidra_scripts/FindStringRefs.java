// Locate string constants in memory and report the code that references them.
//
// MapleStory.exe has a Themida-rebuilt IAT, so cross-referencing by imported API name
// does not work; strings are the reliable way in. Ghidra's auto-analysis does not
// reliably *define* strings in this binary, so this searches raw memory bytes rather
// than the defined-data listing, then looks for references to each hit.
//
// Pass search terms as script arguments; defaults cover the launch tokens.
//
//@category MapleCW

import java.nio.charset.StandardCharsets;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.ReferenceIterator;

public class FindStringRefs extends GhidraScript {

    private static final int MAX_HITS_PER_TERM = 12;
    private static final int MAX_REFS_PER_HIT = 20;

    @Override
    public void run() throws Exception {
        String[] terms = getScriptArgs();
        if (terms == null || terms.length == 0) {
            terms = new String[] { "WEBSTART", "IPPORT", "GAMELAUNCHING", "STEAMSTART" };
        }

        for (String term : terms) {
            if (monitor.isCancelled()) {
                return;
            }
            println("");
            println("################ " + term + " ################");

            // ASCII first, then UTF-16LE, since the client uses both.
            searchFor(term, term.getBytes(StandardCharsets.US_ASCII), "ascii");
            searchFor(term, term.getBytes(StandardCharsets.UTF_16LE), "utf16");
        }
    }

    private void searchFor(String term, byte[] pattern, String label) {
        Address at = currentProgram.getMinAddress();
        int found = 0;

        while (at != null && found < MAX_HITS_PER_TERM && !monitor.isCancelled()) {
            Address hit = find(at, pattern);
            if (hit == null) {
                break;
            }
            found++;
            println("  [" + label + "] found at " + hit);
            reportRefs(hit);

            try {
                at = hit.add(1);
            } catch (Exception e) {
                break;
            }
        }
        if (found == 0) {
            println("  [" + label + "] not found");
        }
    }

    /** References straight to the hit, and to the few bytes around it. */
    private void reportRefs(Address hit) {
        int total = 0;
        for (int delta = 0; delta < 4; delta++) {
            Address probe;
            try {
                probe = hit.add(delta);
            } catch (Exception e) {
                break;
            }
            ReferenceIterator refs = currentProgram.getReferenceManager().getReferencesTo(probe);
            while (refs.hasNext() && total < MAX_REFS_PER_HIT) {
                Reference r = refs.next();
                Address from = r.getFromAddress();
                Function f = getFunctionContaining(from);
                String where = (f == null)
                        ? "<no function>"
                        : f.getName() + " @ " + f.getEntryPoint();
                println("      <- " + from + "  in  " + where + "  (" + r.getReferenceType() + ")");
                total++;
            }
        }
        if (total == 0) {
            println("      (no direct references; likely reached by computed address)");
        }
    }
}
