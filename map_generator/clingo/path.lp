%%% Accessibility verification from left to right

% Predicate for existence of position (X, Y) (between (0,0) and (31,11))
valid(X, Y) :- X =0..width-1, Y=0..height-1.

% Indicate every free position (X, Y) that is above a block.  
stand(X, Y) :- valid(X, Y), block(X, Y-1), not block(X, Y).

% Indicate the positions (X, Y) that are reachable and such that we can stay on (standing on a block)
stay(X, Y) :- path(X, Y, _), stand(X, Y).

% path(X, Y, J) indicates that point (X,Y) is reachable while still having J jumps and is defined by:

% Initial position
path(0, Y, 3) :- stand(0, Y).

% From an accessible position, we can walk right/left if there are no block obstruing passage and there is a block for foot support
path(X, Y, 3) :- valid(X, Y), stay(X-1, Y), stand(X, Y). 
path(X, Y, 3) :- valid(X, Y), stay(X+1, Y), stand(X, Y). 

% From an accessible block, we can jump one up (if there is no block obstruing)
path(X, Y, 3) :- valid(X, Y), stay(X, Y-1), not block(X, Y).

% From an accessible block, we can jump one up and one side (if there is no block obstruing)
%right
path(X, Y, 3) :- valid(X, Y), stay(X-1, Y-1), not block(X, Y), not block(X-1, Y).
path(X, Y, 3) :- valid(X, Y), stay(X-1, Y-1), not block(X, Y), not block(X, Y-1).
%left
path(X, Y, 3) :- valid(X, Y), stay(X+1, Y-1), not block(X, Y), not block(X+1, Y).
path(X, Y, 3) :- valid(X, Y), stay(X+1, Y-1), not block(X, Y), not block(X, Y-1).

% From an accessible block, we can jump one up and two side (if there is no block obstruing)
%right
path(X, Y, 3) :- valid(X, Y), stay(X-2, Y-1), not block(X, Y), not block(X-1, Y), not block(X-2, Y).
path(X, Y, 3) :- valid(X, Y), stay(X-2, Y-1), not block(X, Y), not block(X-1, Y), not block(X-1, Y-1).
%left
path(X, Y, 3) :- valid(X, Y), stay(X+2, Y-1), not block(X, Y), not block(X+1, Y), not block(X+2, Y).
path(X, Y, 3) :- valid(X, Y), stay(X+2, Y-1), not block(X, Y), not block(X+1, Y), not block(X+1, Y-1).


% From an accessible position, if I touch the side of a block, I can use it to jump.
path(X, Y, J) :- J=0..3, valid(X, Y), path(X, Y-1, J+1), block(X+1, Y-1), not block(X, Y).
path(X, Y, J) :- J=0..3, valid(X, Y), path(X+1, Y-1, J+1), block(X+2, Y-1), not block(X, Y).
path(X, Y, J) :- J=0..3, valid(X, Y), path(X-1, Y-1, J+1), block(X, Y-1), not block(X, Y).


% I can walk into the void
path(X, Y, 3) :- valid(X, Y), stay(X-1, Y), not block(X, Y). 
path(X, Y, 3) :- valid(X, Y), stay(X+1, Y), not block(X, Y). 

% I can fall downwards
path(X, Y, J) :- valid(X, Y), path(X, Y+1, J), not block(X, Y). 

% I can fall downwards going left/right
path(X, Y, J) :- valid(X, Y), path(X-1, Y+1, J), not block(X, Y), not block(X-1, Y), not block(X-1, Y-1). 
path(X, Y, J) :- valid(X, Y), path(X+1, Y+1, J), not block(X, Y), not block(X+1, Y), not block(X+1, Y-1). 



victory :- path(width-1, _, _).

% #show path/3.
% #show victory/0.
