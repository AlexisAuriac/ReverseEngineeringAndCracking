# ez_breezy

open http://reverse.blackfoot.io:1330/ whith google chrome (firefox doesn't work).

go to **Debugger** and open index.wasm.

flag is in clear a the end of the file.

flag: ez_br33zy!

```
(module
  (memory $a.memory (;0;) (import "a" "memory") 256 256)
  (global $global0 (mut i32) (i32.const 5244432))
  (func $verifyPassword (;0;) (export "b") (param $var0 i32) (result i32)
    local.get $var0
    call $strcmp
    i32.eqz
  )
  (func $strcmp (;1;) (param $var0 i32) (result i32)
    (local $var1 i32)
    (local $var2 i32)
    (local $var3 i32)
    i32.const 1024
    local.set $var1
    block $label0 (result i32)
      loop $label1
        local.get $var1
        i32.load8_s
        local.set $var2
        i32.const 0
        local.get $var0
        i32.load8_u
        local.tee $var3
        i32.eqz
        br_if $label0
        drop
        local.get $var2
        i32.const 255
        i32.and
        local.get $var3
        i32.eq
        if
          local.get $var1
          i32.const 1
          i32.add
          local.set $var1
          local.get $var0
          i32.const 1
          i32.add
          local.set $var0
          br $label1
        end
      end $label1
      local.get $var3
      i32.const 24
      i32.shl
      i32.const 24
      i32.shr_s
    end $label0
    local.get $var2
    i32.sub
  )
  (func $stackSave (;2;) (export "c") (result i32)
    global.get $global0
  )
  (func $stackRestore (;3;) (export "e") (param $var0 i32)
    local.get $var0
    global.set $global0
  )
  (func $stackAlloc (;4;) (export "d") (param $var0 i32) (result i32)
    global.get $global0
    local.get $var0
    i32.sub
    i32.const -16
    i32.and
    local.tee $var0
    global.set $global0
    local.get $var0
  )
  (func $__wasm_call_ctors (;5;) (export "a")
    nop
  )
  (data (i32.const 1024) "ez_br33zy!") <----- Flag
)
```