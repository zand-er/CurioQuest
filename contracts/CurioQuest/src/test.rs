#![cfg(test)]

use soroban_sdk::{Env, Address};
use crate::{CurioQuest, CurioQuestClient};

#[test]
fn test_happy_path() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CurioQuest);
    let client = CurioQuestClient::new(&env, &contract_id);

    let user = Address::random(&env);

    client.ask_question(&user);
    client.complete_quiz(&user, &90);

    let (_, _, points) = client.get_progress(&user);
    assert_eq!(points, 10);
}

#[test]
fn test_low_score_edge() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CurioQuest);
    let client = CurioQuestClient::new(&env, &contract_id);

    let user = Address::random(&env);

    client.complete_quiz(&user, &60);

    let (_, _, points) = client.get_progress(&user);
    assert_eq!(points, 5);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CurioQuest);
    let client = CurioQuestClient::new(&env, &contract_id);

    let user = Address::random(&env);

    client.ask_question(&user);
    client.ask_question(&user);
    client.complete_quiz(&user, &80);

    let (q, c, _) = client.get_progress(&user);

    assert_eq!(q, 2);
    assert_eq!(c, 1);
}

#[test]
fn test_multiple_quizzes() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CurioQuest);
    let client = CurioQuestClient::new(&env, &contract_id);

    let user = Address::random(&env);

    client.complete_quiz(&user, &90);
    client.complete_quiz(&user, &90);

    let (_, _, points) = client.get_progress(&user);
    assert_eq!(points, 20);
}

#[test]
fn test_zero_progress() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CurioQuest);
    let client = CurioQuestClient::new(&env, &contract_id);

    let user = Address::random(&env);

    let (q, c, p) = client.get_progress(&user);

    assert_eq!(q, 0);
    assert_eq!(c, 0);
    assert_eq!(p, 0);
}

