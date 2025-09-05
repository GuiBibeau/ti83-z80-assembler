; math_demo.asm - Display "Hello World!" and show 2+2=4
; For TI-83 Plus calculator

.org $9D93
.db $BB,$6D

    ; Clear screen and display "Hello World!"
    bcall(_ClrLCDFull)      ; Clear the screen
    bcall(_HomeUp)          ; Move cursor to home position
    ld hl,hello_msg         ; Load address of hello message
    bcall(_PutS)            ; Display the string
    bcall(_NewLine)         ; Move to next line
    
    ; Calculate 2 + 2
    ld a,2                  ; Load 2 into accumulator
    add a,2                 ; Add 2 to accumulator (a = 4)
    
    ; Display "2 + 2 = "
    ld hl,math_msg          ; Load address of math message
    bcall(_PutS)            ; Display the string
    
    ; Convert result to display
    ; Since we know it's 4, we can just add '0' to convert to ASCII
    add a,'0'               ; Convert 4 to ASCII '4' (0x34)
    bcall(_PutC)            ; Display the character
    
    ; Add some space and return
    bcall(_NewLine)         ; Move to next line
    bcall(_NewLine)         ; Add some space
    ret                     ; Return to TI-OS

; Data section with messages
hello_msg:
    .db "Hello World!",0    ; Null-terminated string

math_msg:
    .db "2 + 2 = ",0        ; Null-terminated string