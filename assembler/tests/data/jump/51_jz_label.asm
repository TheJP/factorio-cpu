    CMP A, 0
    JZ skip
    MOV B, 1
skip:
    CMP A, 1
    JE skip2
    MOV C, 1
skip2:
    HALT ; B = 0, C = 1
