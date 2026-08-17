// Print the instruction listing for a VA range, re-disassembling if Ghidra has none.
//
//   -postScript DumpAsm.java <out-file> 141b3fb10 0x120
//
// Written for the case DecompileFunc cannot handle: a function Ghidra truncates with
// "bad instruction data". Decompiler output stops there and says nothing about the rest,
// so the bytes have to be read directly. Prints raw hex alongside each instruction so a
// bad disassembly is visible as such rather than being taken at face value.
//
//@category MapleCW

import java.io.PrintWriter;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.mem.MemoryBlock;

public class DumpAsm extends GhidraScript {

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args == null || args.length < 2) {
            println("usage: DumpAsm <out-file> <hex-address> [length]");
            return;
        }

        PrintWriter out = new PrintWriter(args[0], "UTF-8");
        Address start = currentProgram.getAddressFactory().getAddress(args[1].replace("0x", ""));
        long length = args.length > 2 ? Long.decode(args[2]) : 0x100;

        MemoryBlock block = currentProgram.getMemory().getBlock(start);
        out.println("// " + start + " .. " + start.add(length - 1)
                + "   block=" + (block == null ? "none" : block.getName()));

        byte[] raw = new byte[(int) length];
        currentProgram.getMemory().getBytes(start, raw);

        Address addr = start;
        Address end = start.add(length);
        while (addr.compareTo(end) < 0 && !monitor.isCancelled()) {
            Instruction insn = getInstructionAt(addr);
            if (insn == null) {
                // No code here yet - ask for it. If the bytes really are data this
                // returns nothing and the byte dump below is the only honest answer.
                disassemble(addr);
                insn = getInstructionAt(addr);
            }
            if (insn == null) {
                StringBuilder hex = new StringBuilder();
                int off = (int) addr.subtract(start);
                for (int i = off; i < Math.min(off + 16, raw.length); i++) {
                    hex.append(String.format("%02x ", raw[i]));
                }
                out.println(String.format("%s  %-48s  <no instruction>", addr, hex.toString()));
                addr = addr.add(1);
                continue;
            }
            StringBuilder hex = new StringBuilder();
            for (byte b : insn.getBytes()) {
                hex.append(String.format("%02x ", b));
            }
            out.println(String.format("%s  %-32s  %s", addr, hex.toString(), insn.toString()));
            addr = addr.add(insn.getLength());
        }

        out.flush();
        out.close();
        println("wrote " + args[0]);
    }
}
