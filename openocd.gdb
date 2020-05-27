target remote :3333
set print asm-demangle on
set print pretty on

load

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind
break main
continue
continue