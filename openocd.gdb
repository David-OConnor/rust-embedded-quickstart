target remote :3333
set print asm-demangle on
set print pretty on
monitor arm semihosting enable
load

# Detect unhandled exceptions, hard faults and panics
break DefaultHandler
break main
continue
continue