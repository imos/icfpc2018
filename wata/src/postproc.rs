#![allow(unused)]
use *;
use std::collections::*;


fn fusion_all(matrix: V3<bool>, positions: Vec<P>) {
    let r = matrix.len();
    let mut cmdss: Vec<VecDeque<Command>> = Vec::new();
    {
        let filled_func = |p: P| { matrix[p] };
        let goal_func = |p: P| { p.x == 0 && p.y == 0 && p.z == 0 };
        for &pos in positions.iter() {
            let mut bfs = bfs::BFS::new(r);
            let ret = bfs.bfs(filled_func, &vec![pos], goal_func);
            eprintln!("{:?}", ret);
            let cmds = bfs.restore(ret.unwrap());
            cmdss.push(cmds.into_iter().collect());
        }
        eprintln!("{:?}", cmdss);
    }

    let mut positions = positions;

    //let mut sim = sim::SimState::from_positions(matrix, positions);
    let mut occupied = InitV3::new(false, r);
    loop {
        occupied.init();
        /*
        let mut step_cmds = Vec::new();
        for cmds in cmdss.iter_mut() {
            step_cmds.push(cmds.pop_front().unwrap_or(Command::Wait));
        }
        if step_cmds.iter().all(|&v| v == Command::Wait) {
            break;
        }
        */

        for &pos in positions.iter() {
            occupied[pos] = true;
        }

        let mut all_orz = true;
        for (mut pos, mut cmds) in positions.iter_mut().zip(cmdss.iter_mut()) {
            let cmd = cmds.pop_front().unwrap_or(Command::Wait);
            let mut orz = false;
            for (p, c) in path(*pos, cmd) {
                if occupied[p] {
                    cmds.push_front(cmd);
                    orz = true;
                    break;
                }
                occupied[p] = true;
                *pos = p;
            }
            if !orz {
                all_orz = false;
            }
        }
        if all_orz {
            break;
        }
    }
}


fn path(mut p: P, mut cmd: Command) -> Vec<(P, Command)> {
    // (next_pos, current_cmd)
    let mut ret = Vec::new();
    while let Command::LMove(d1, d2) = cmd {
        let v = d1 / d1.mlen();
        p += v;
        ret.push((p, cmd));
        let d1 = d1 - v;
        cmd = if d1.mlen() == 0 {
            Command::SMove(d2)
        } else {
            Command::LMove(d1, d2)
        };
    }
    while let Command::SMove(d1) = cmd {
        let v = d1 / d1.mlen();
        p += v;
        ret.push((p, cmd));
        let d1 = d1 - v;
        cmd = if d1.mlen() == 0 {
            Command::Wait
        } else {
            Command::SMove(d1)
        };
    }
    ret
}