JMP start

; Interrupts
JMP i1 ; [02]
JMP i2 ; [03]

start:
    MOV A, 2
    INT A
    INT A
    HALT ; A = 3, B = 42

; Functions
i1:
    ADD A, 1
    RET

i2:
    MOV B, 42
    RET
