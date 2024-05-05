# ---
# Project: Kairos
# File: Makefile
# ---

MAIN		= 		src/main.rs

NAME   		=		kairos

CC 			= 		cargo build

CFLAGS 		= 		-Wall -Wextra -g3

all:	$(NAME)

$(NAME):
			$(CC)
			cp target/debug/$(NAME) .

clean:
			cargo clean

fclean: clean
			$(RM) $(NAME)

re:     fclean all

.PHONY:		all fclean clean re
