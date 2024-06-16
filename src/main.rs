
use bracket_lib::prelude::*;
use lazy_static::lazy_static;

//游戏三种模式
enum GameMode{
    Menu,
    Playing,
    End,
}

struct DefaultParameters{
    screen_width:i32,
    screen_height:i32,
    frame_duration:f32,
}

lazy_static!{
    static ref DEFAULT_PARAMETERS:DefaultParameters = DefaultParameters{
        screen_width:80,
        screen_height:50,
        frame_duration:75.0,
    };
}


struct State{
    player:Player,
    frame_time:f32,
    mode:GameMode,
    score:i32,
    obstacle:Obstacle,
}

struct Player{
    ////游戏世界空间，为无限大
    x:i32,
    y:i32,
    velocity:f32,
}
struct Obstacle{
    //游戏世界空间，为无限大
    x:i32,
    //障碍物的空余空间
    gap_y:i32,
    size:i32,
}
impl Obstacle{
    fn new(x:i32,score:i32) -> Self{
        let mut random = RandomNumberGenerator::new();
        Self{
            x,
            gap_y:random.range(10, 40),
            size:i32::max(2, 20-score),
        }
    }
    fn render(&mut self, ctx:&mut BTerm, player_x:i32){
        //屏幕空间
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        for y in 0..self.gap_y - half_size{
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        for y in self.gap_y + half_size..DEFAULT_PARAMETERS.screen_height{
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn hit_obstacle(&self, player:&Player) -> bool{
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}  
impl State{
    fn new() -> Self{
        //创建实例
        Self{
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode:GameMode::Menu,
            score:0,
            obstacle:Obstacle::new(DEFAULT_PARAMETERS.screen_width, 0),
        }
    }
    fn playing(&mut self,ctx:&mut BTerm){
        //清除屏幕背景并设置新的背景颜色。
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > DEFAULT_PARAMETERS.frame_duration{
            self.player.gravity_to_move();
        }

        //按下空格键时，飞起
        if let Some(VirtualKeyCode::Space) = ctx.key{
            self.player.flap();
        }
        
        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        //如果穿过障碍物，分数加1
        if self.player.x > self.obstacle.x{
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + DEFAULT_PARAMETERS.screen_width, self.score);
        }

        //判断是否碰到边界或障碍物
        if self.player.y > DEFAULT_PARAMETERS.screen_height || self.obstacle.hit_obstacle(&self.player){
            self.mode = GameMode::End;
        }
    }
    fn restart(&mut self){
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0;
        self.obstacle = Obstacle::new(DEFAULT_PARAMETERS.screen_width, 0);
    }
    fn end(&mut self,ctx:&mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        //判断是否输入P或Q
        match ctx.key{
            None => {}
            Some(key) => {
                match key{
                    VirtualKeyCode::P => self.restart(),
                    VirtualKeyCode::Q => ctx.quitting = true,
                    _ => {}
                }
            }
        }
    }
    fn main_menu(&mut self,ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        //判断是否输入P或Q
        match ctx.key{
            None => {}
            Some(key) => {
                match key{
                    VirtualKeyCode::P => self.restart(),
                    VirtualKeyCode::Q => ctx.quitting = true,
                    _ => {}
                }
            }
        }

    }
}
impl Player{
    fn new(x:i32,y:i32) -> Self{
        Self{
            x,
            y,
            velocity:0.0,
        }
    }
    fn render(&mut self, ctx:&mut BTerm){
        //设置一个位于 (0, self.y) 位置的图形或文本的前景色为黄色，背景色为黑色，并显示一个 '@' 字符
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    //设置重力
    fn gravity_to_move(&mut self) {
        if self.velocity < 2.0{
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
    //点击空格往上飞
    fn flap(&mut self){
        self.velocity = -2.0;

    }
}

impl GameState for State{
    fn tick(&mut self,ctx:&mut BTerm){
        match self.mode{
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.playing(ctx),
            GameMode::End => self.end(ctx)
        }

    }
}


fn main() -> BError{
    let context = BTermBuilder::simple80x50()
        .with_title("FLappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
