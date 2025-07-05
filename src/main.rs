use std::cell::LazyCell;

use anyhow::Error;
use discord::{model::{ChannelId, Message, ReactionEmoji}, Discord, GetMessages};
use serde_derive::Serialize;

//const JQNPR_CHANNEL_ID: ChannelId = ChannelId(1075791895416012860);

const REACTION_RED : LazyCell<ReactionEmoji> = LazyCell::new(||ReactionEmoji::Unicode("🔴".to_string()));
const REACTION_GREEN : LazyCell<ReactionEmoji> = LazyCell::new(||ReactionEmoji::Unicode("🟢".to_string()));
const REACTION_ORANGE : LazyCell<ReactionEmoji> = LazyCell::new(||ReactionEmoji::Unicode("🟠".to_string()));
const REACTION_COOKIE : LazyCell<ReactionEmoji> = LazyCell::new(||ReactionEmoji::Unicode("🍪".to_string()));
const REACTION_PEANUT : LazyCell<ReactionEmoji> = LazyCell::new(||ReactionEmoji::Unicode("🥜".to_string()));

#[derive(Clone, Debug, Serialize)]
struct Jqnpr {
    id: u64,
    image_url: String, 
    reponse: Vec<String>,
    auteur: String,
    date: String,
    reactions_image: Vec<(String, u64)>,
    reactions_reponse: Vec<(String, u64)>,
    indices: Vec<String>,
    mentions: Vec<String>
}

fn main() -> Result<(), Error> {
    let discord_token = std::env::var("DISCORD_TOKEN").expect("La variable DISCORD_TOKEN doit être définie");
    let channel_id = std::env::var("DISCORD_CHANNEL").expect("La variable DISCORD_CHANNEL doit être définie");
    let channel_id = u64::from_str_radix(&channel_id, 10).expect("nombre invalide dans la variable DISCORD_CHANNEL");
    let channel_id = ChannelId(channel_id);
    let cnx = Discord::from_user_token(&discord_token).unwrap();
    let mut messages = Vec::<Message>::new();
    let mut next_pack = GetMessages::MostRecent;
    loop {
        match cnx.get_messages(channel_id, next_pack, Some(100)){
            Ok(mut msg_pack) => {
                if let Some(last) = msg_pack.last(){
                    eprintln!("Lit un pack de {} messages à partir de {}.", msg_pack.len(), last.timestamp.to_rfc3339());
                    next_pack = GetMessages::Before(last.id);
                }
                else {
                    eprintln!("Plus de messages.");
                    break;
                }    
                messages.append(&mut msg_pack);
            }
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    };
    messages.reverse();
    let jqneprs = parse(&messages);
    let json = serde_json::to_string_pretty(&jqneprs)?;
    println!("{}", &json);
    Ok(())
}

fn parse(messages: &Vec<Message>) -> Vec<Jqnpr> {  
    let mut parsed = Vec::<Jqnpr>::new();
    let mut current = Option::<Jqnpr>::None;
    let mut next = false;
    for msg in messages.iter() {
        // Pastille rouge
        if msg.reactions.iter().any(|r| r.emoji == *REACTION_RED) {
            if let Some(attachement) = msg.attachments.first(){
                current = Some( Jqnpr{
                    id: msg.id.0,
                    image_url: attachement.url.clone(),
                    reponse: Vec::new(),
                    auteur: msg.author.name.clone(),
                    date: msg.timestamp.to_rfc3339(),
                    reactions_image: get_reactions(msg),
                    reactions_reponse: Vec::new(),
                    indices: Vec::new(),
                    mentions: Vec::new(),
                });
            }
        }

        if let Some(ref mut jqnpr) = current{
            // Pastille verte
            if msg.reactions.iter().any(|r| r.emoji == *REACTION_GREEN) {
                jqnpr.reponse.push(msg.content.clone());
                jqnpr.reactions_reponse.append(&mut get_reactions(msg));
                next = true;
            }
            // Pastille Orange
            if msg.reactions.iter().any(|r| r.emoji == *REACTION_ORANGE) {
                jqnpr.indices.push(msg.content.clone());
            }
            // Cookies & Peanuts
            if msg.reactions.iter().any(|r| r.emoji == *REACTION_PEANUT || r.emoji == *REACTION_COOKIE) {
                jqnpr.mentions.push(msg.content.clone());
            }
        }
        if next {
            parsed.push(current.unwrap());
            current = None;
            next = false;
        }
    }
    if let Some(jqnpr) = current {
        parsed.push(jqnpr);
    }
    parsed
}

fn get_reactions(msg: &Message) -> Vec<(String,u64)>{
    msg.reactions.iter()
    .filter(|r| r.emoji != *REACTION_RED && r.emoji != *REACTION_GREEN)
    .map(|r| match &r.emoji {
        ReactionEmoji::Unicode(u) => (u.clone(), r.count),
        ReactionEmoji::Custom { name, id: _ } => (name.clone(), r.count)
    })
    .collect()
}