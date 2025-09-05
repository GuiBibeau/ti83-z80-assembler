; advanced.asm - Test advanced Z80 instructions
.org $9D93
.db $BB,$6D

    ; Test IX/IY registers
    ld ix,$8000         ; Load IX with address
    ld iy,buffer        ; Load IY with buffer address
    ld a,(ix+0)        ; Load from indexed address
    ld (iy+1),a        ; Store to indexed address
    
    ; Test bit manipulation
    ld a,$55           ; Load test pattern 01010101
    bit 7,a            ; Test bit 7
    jr z,skip1         ; Jump if zero
    set 6,a            ; Set bit 6
    res 0,a            ; Reset bit 0
skip1:
    rlc a              ; Rotate left circular
    sla a              ; Shift left arithmetic
    
    ; Test block transfer
    ld hl,source       ; Source address
    ld de,dest        ; Destination address  
    ld bc,10          ; Count
    ldir              ; Block copy
    
    ; Test I/O ports (LCD control)
    ld a,$03          ; LCD command
    out ($10),a       ; Send to LCD command port
    in a,($11)        ; Read LCD status
    
    ; Test expanded ROM calls
    bcall(_ClrLCDFull)    ; Clear screen
    ld hl,message         ; Load message address
    bcall(_PutS)          ; Display string
    bcall(_GetKey)        ; Wait for key
    
    ; Math operations
    bcall(_FPAdd)         ; Floating point add
    bcall(_DispHL)        ; Display HL value
    
    ret

source:
    .db "Source",0
    
dest:
    .db "      ",0
    
buffer:
    .db 0,0,0,0,0,0,0,0
    
message:
    .db "Advanced Test",0