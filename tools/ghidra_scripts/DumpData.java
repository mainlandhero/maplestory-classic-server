// Print the data at an address as bytes, ASCII and UTF-16.
//
//   -postScript DumpData.java <out-file> 1433881d0 [more addresses...]
//
// The client's UI control names are wide-string literals that Ghidra often leaves as
// unnamed DAT_ symbols, so a button handler reads as `FUN_142aa1a20(ctl, &DAT_1433881d0)`
// and says nothing about which button it is. This turns that back into a name.
//
//@category MapleCW

import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;

public class DumpData extends GhidraScript {

    private static final int LENGTH = 96;

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args == null || args.length < 2) {
            println("usage: DumpData <out-file> <hex-address> [...]");
            return;
        }

        PrintWriter out = new PrintWriter(args[0], "UTF-8");
        for (int i = 1; i < args.length; i++) {
            Address addr = currentProgram.getAddressFactory().getAddress(args[i].replace("0x", ""));
            byte[] raw = new byte[LENGTH];
            try {
                currentProgram.getMemory().getBytes(addr, raw);
            } catch (Exception e) {
                out.println(addr + "  <unreadable: " + e.getMessage() + ">");
                continue;
            }

            StringBuilder hex = new StringBuilder();
            for (int j = 0; j < 32; j++) {
                hex.append(String.format("%02x ", raw[j]));
            }
            out.println("=== " + addr);
            out.println("  hex   " + hex);
            out.println("  ascii " + printable(new String(raw, StandardCharsets.US_ASCII)));
            out.println("  utf16 " + printable(new String(raw, StandardCharsets.UTF_16LE)));
            out.println();
        }
        out.flush();
        out.close();
        println("wrote " + args[0]);
    }

    /** Stop at the first NUL and keep only characters that survive a log file. */
    private String printable(String s) {
        StringBuilder b = new StringBuilder();
        for (char c : s.toCharArray()) {
            if (c == 0) {
                break;
            }
            b.append(c >= 0x20 && c < 0x7f ? c : '.');
        }
        return b.toString();
    }
}
