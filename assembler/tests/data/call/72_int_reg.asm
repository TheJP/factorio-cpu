; Interrupts
JMP i1
JMP i2

INT A
INT A
HALT ; A = 1, B = 42

; Functions
i1:
    ADD A, 1
    RET

i2:
    MOV B, 42
    RET
