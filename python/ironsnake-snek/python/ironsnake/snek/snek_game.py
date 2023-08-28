import pygame
import random

# Initialize pygame
pygame.init()

# Colors
WHITE = (255, 255, 255)
GREEN = (0, 255, 0)
RED = (255, 0, 0)
BLACK = (0, 0, 0)

# Game dimensions
WIDTH, HEIGHT = 640, 480
CELL_SIZE = 20

START_SPEED = 8

screen = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.display.set_caption('Snek Game')

# Direction constants
LEFT = (-1, 0)
RIGHT = (1, 0)
UP = (0, -1)
DOWN = (0, 1)


class Snake:
    def __init__(self):
        self.body = [(5, 5), (4, 5), (3, 5)]
        self.direction = RIGHT
        self.grow = False

    def move(self):
        head_x, head_y = self.body[0]
        new_dir_x, new_dir_y = self.direction
        new_head = (head_x + new_dir_x, head_y + new_dir_y)

        if self.grow:
            self.body = [new_head] + self.body
            self.grow = False
        else:
            self.body = [new_head] + self.body[:-1]

    def grow_snake(self):
        self.grow = True

    def collides_with_itself(self):
        return self.body[0] in self.body[1:]


class Food:
    def __init__(self):
        self.position = (random.randint(0, (WIDTH // CELL_SIZE) - 1), random.randint(0, (HEIGHT // CELL_SIZE) - 1))

    def randomize_position(self):
        self.position = (random.randint(0, (WIDTH // CELL_SIZE) - 1), random.randint(0, (HEIGHT // CELL_SIZE) - 1))


def draw_snake(snake):
    for segment in snake.body:
        pygame.draw.rect(screen, GREEN, (segment[0] * CELL_SIZE, segment[1] * CELL_SIZE, CELL_SIZE, CELL_SIZE))


def draw_food(food):
    pygame.draw.rect(screen, RED, (food.position[0] * CELL_SIZE, food.position[1] * CELL_SIZE, CELL_SIZE, CELL_SIZE))


def display_score(score):
    font = pygame.font.Font(None, 36)
    text = font.render(f"Score: {score}", True, BLACK)
    screen.blit(text, (10, 10))


def display_lives(lives):
    font = pygame.font.Font(None, 36)
    text = font.render(f"Lives: {lives}", True, BLACK)
    screen.blit(text, (WIDTH - 100, 10))


def game_over(score):
    screen.fill(WHITE)

    font_large = pygame.font.Font(None, 72)
    text_game_over = font_large.render("Game Over!", True, BLACK)
    game_over_rect = text_game_over.get_rect(center=(WIDTH // 2, HEIGHT // 2 - 60))

    font_medium = pygame.font.Font(None, 48)
    text_score = font_medium.render(f"Score: {score}", True, BLACK)
    score_rect = text_score.get_rect(center=(WIDTH // 2, HEIGHT // 2))

    font_small = pygame.font.Font(None, 36)
    text_replay = font_small.render("Press 'R' to play again or 'ESC' to exit.", True, BLACK)
    replay_rect = text_replay.get_rect(center=(WIDTH // 2, HEIGHT // 2 + 60))

    screen.blit(text_game_over, game_over_rect.topleft)
    screen.blit(text_score, score_rect.topleft)
    screen.blit(text_replay, replay_rect.topleft)
    pygame.display.flip()

    waiting_for_input = True
    while waiting_for_input:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit(0)
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r:
                    main()
                elif event.key == pygame.K_ESCAPE:
                    pygame.quit()
                    exit(0)


def main():
    snake = Snake()
    food = Food()
    clock = pygame.time.Clock()
    score = 0
    lives = 3
    speed = START_SPEED
    running = True
    snek_eat_frames = 0
    messages = ["Such Snek!", "Much Wow!", "Very Eat!", "So Score!", "Amaze!", "Much Tasty!"]
    current_message = ""

    while running:
        screen.fill(WHITE)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_LEFT and snake.direction != RIGHT:
                    snake.direction = LEFT
                elif event.key == pygame.K_RIGHT and snake.direction != LEFT:
                    snake.direction = RIGHT
                elif event.key == pygame.K_UP and snake.direction != DOWN:
                    snake.direction = UP
                elif event.key == pygame.K_DOWN and snake.direction != UP:
                    snake.direction = DOWN

        snake.move()

        # Check for collisions
        if snake.body[0] == food.position:
            score += 1
            snake.grow_snake()
            food.randomize_position()
            speed = min(30, speed + 1)  # Increase speed, but max out at 20
            snek_eat_frames = 20
            current_message = random.choice(messages)

        # Check if snake hits the wall or its tail
        head_x, head_y = snake.body[0]
        if (
                head_x < 0 or head_x >= WIDTH // CELL_SIZE or
                head_y < 0 or head_y >= HEIGHT // CELL_SIZE or
                snake.collides_with_itself()
        ):
            lives -= 1
            if lives == 0:
                game_over(score)
                running = False
            else:
                snake = Snake()  # Reset snake
                speed = START_SPEED  # Reset speed

        draw_snake(snake)
        draw_food(food)
        display_score(score)
        display_lives(lives)

        if snek_eat_frames > 0:
            font = pygame.font.Font(None, 72)
            text_surface = font.render(current_message, True, BLACK)
            text_rect = text_surface.get_rect(center=(WIDTH // 2, HEIGHT // 2))
            screen.blit(text_surface, text_rect.topleft)
            snek_eat_frames -= 1

        pygame.display.flip()

        clock.tick(speed)

    pygame.quit()


if __name__ == "__main__":
    main()
