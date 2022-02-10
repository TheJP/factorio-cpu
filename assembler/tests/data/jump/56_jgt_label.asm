    CMP A, 0
    JGT skip
    MOV B, 1
skip:
    CMP A, -1
    JGT skip2
    MOV C, 1
skip2:
    HALT ; B = 1, C = 0
