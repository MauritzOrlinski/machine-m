# IDEA: Store running sum of squares (quad sum) in S0 and number of read elements (N) in S1
# Init cells S0 and S1
CONST(0)
STORE(0)
STORE(1)
# both S0 and S1 are zero
# label1:
READ 
NEGATIVE(19)
# branch1: Update quad sum and count N
STORE(3)
MULT(3) # R is the square of read value
ADD(0)
STORE(0) # N is now sum of all squares of read values
CONST(1)
ADD(1)
STORE(1)
JUMP(7) # Jump to label1
# branch2: finalize computation and print out: 
RECALL(0)
DIV(1)
SQRT
WRITE "Quad-Mean:"
PRINT
RECALL(1)
WRITE "Numbers read:"
PRINT
STOP
