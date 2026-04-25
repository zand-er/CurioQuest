#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Map, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Progress {
    pub questions: u32,
    pub quizzes: u32,
    pub points: u32,
}

#[contract]
pub struct CurioQuest;

#[contractimpl]
impl CurioQuest {

    // Ask a question (increments count)
    pub fn ask_question(env: Env, user: Address) {
        let key = Symbol::short("PROG");

        let mut store: Map<Address, Progress> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let mut prog = store.get(user.clone()).unwrap_or(Progress {
            questions: 0,
            quizzes: 0,
            points: 0,
        });

        prog.questions += 1;

        store.set(user.clone(), prog);
        env.storage().instance().set(&key, &store);
    }

    // Complete quiz and earn points
    pub fn complete_quiz(env: Env, user: Address, score: u32) {
        let key = Symbol::short("PROG");

        let mut store: Map<Address, Progress> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let mut prog = store.get(user.clone()).unwrap_or(Progress {
            questions: 0,
            quizzes: 0,
            points: 0,
        });

        prog.quizzes += 1;

        if score >= 80 {
            prog.points += 10;
        } else {
            prog.points += 5;
        }

        store.set(user.clone(), prog);
        env.storage().instance().set(&key, &store);
    }

    // Get progress
    pub fn get_progress(env: Env, user: Address) -> (u32, u32, u32) {
        let key = Symbol::short("PROG");

        let store: Map<Address, Progress> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let prog = store.get(user).unwrap_or(Progress {
            questions: 0,
            quizzes: 0,
            points: 0,
        });

        (prog.questions, prog.quizzes, prog.points)
    }
}
