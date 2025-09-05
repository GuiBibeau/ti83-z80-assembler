; hello.asm - Display "Hello World!" on TI-83 Plus
.org $9D93
.db $BB,$6D
    bcall(_ClrLCDFull)  ; Clear the screen
    bcall(_HomeUp)      ; Move cursor to home position
    ld hl,message       ; Load address of message
    bcall(_PutS)        ; Display the string
    ret                 ; Return to TI-OS

message:
    .db "Hello World!",0 ; Null-terminated string