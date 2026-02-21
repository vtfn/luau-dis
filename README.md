# luau-dis
High performance luau bytcode parser and disassembler.

# How to use

```rust
// Parse raw luau bytes.
let bytecode = Bytecode::parse(...)?;

// Create a Luac formatter buffer.
let mut fmt = LuacFormatter::new();

for p in bytecode.protos() {
    let mut dec = Decoder::new(p.instructions);

    while let Ok(ins) = dec.decode() {
        // Formatting an instruction does not allocate.
        fmt.format(&ins);
        println!(w, "{}", fmt.as_str());
    }
}
```

# Example Output

```luau
local x = 5

local function something(num)
    print(x)
    num = 10
    print(num)
end

something(8)
x = 2
something(4)
```
```
"something"
   GETIMPORT 1, 1, 1073741824
   GETUPVAL 2, 0
   CALL 1, 2, 1
   LOADN 0, 10
   GETIMPORT 1, 1, 1073741824
   MOVE 2, 0
   CALL 1, 2, 1
   RETURN 0, 1
"anonymous"
   PREPVARARGS 0
   LOADN 0, 5
   NEWCLOSURE 1, 0
   CAPTURE 1, 0
   MOVE 2, 1
   LOADN 3, 8
   CALL 2, 2, 1
   LOADN 0, 2
   MOVE 2, 1
   LOADN 3, 4
   CALL 2, 2, 1
   CLOSEUPVALS 0
   RETURN 0, 1
```
