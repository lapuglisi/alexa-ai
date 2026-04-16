use std::{collections::HashMap, hash::Hash, sync::LazyLock};

pub const TROCADILHOS: [&str; 25] = [
  "Em buraco de cobra, tatu caminha dentro.",
  "Queria ser legal assim como você.",
  "Como sempre você com essa camisa.",
  "Você está gozado hoje.",
  "Você está metido hoje.",
  "Você vai ter que se virar e aguentar a barra.",
  "O CEO da empresa se chama Seu Kuke Myiama.",
  "Você não gosta de café expresso, porque no cuador é mais forte.",
  "Em casa, você tem tomada atrás do sofá?
	 Por quê, você mexe com força?
	 Não, só em fio grosso.",
  "Gosta de jogos de tabuleiro? Você tem dado em casa?",
  "Nesse calor, até marinheiro em terra firme na boom da sua.",
  "Tem um índio sentado no asfalto e outro sentado na grama. Qual dos dois tem terra na boomda?",
  "Você gosta de suco de umbu?
	Rapaz, eu nunca vi umbu ser tão azedo.",
  "Você está bem hoje?
	 Bom... não como você, mas gostaria.",
  "Você sabe fazer vitamina?
	 Sim, por quê?
	 Então bate uma pra mim.
	 Claro, qual você quer?
	 Só caqui com mamão na banana.",
  "Você sabe tocar violão?
	  Sim!
		Então toca uma pra mim.",
  "Pessoal, hoje na aula eu quero que vocês façam grupos de quatro",
  "Nome com duplo sentido:	Jacinto Leite Aquino Rêgo.",
  "Nome com duplo sentido:	Sophie Zannál.",
  "Nome com duplo sentido:	Deide Costa.",
  "Nome com duplo sentido:	Daniel Confuego Nargóla.",
  "Nome com duplo sentido:	João Cabral Que Melo Neto.",
  "Nome com duplo sentido:	Vanessa Fadinha.",
  "Nome com duplo sentido:	Tomás Turbano Pinto Maior.",
  "Nome com duplo sentido:	Paula Nabussa.",
];

pub const DESERVED_RESPONSES: LazyLock<HashMap<&'static str, &str>> = LazyLock::new(|| {
  let mut map = HashMap::new();

  map.insert("(ão|ao)+[!\\.]?$", "Meu pau na sua mão");
  map.insert("(au|al)+[!\\?\\.]?$", "Teu cool no meu pau.");
  map.insert("(iste)[!\\.\\?]?$", "Meu pau em riste!");
  map.insert(
    "(cala|calar).*(boca)",
    "calar a boquinha já morreu, senta com força e pega no meu.",
  );
  map.insert("(ente)[!\\.\\?]?$", "Meu pau você bem sente.");
  map.insert("([ctj]u|ool|ul)[!\\.\\?]?$", "Meu pau no seu cool.");
  map.insert("(ol)[\\.\\?!]?$", "Teu cool no meu anzol.");
  map.insert("(el|éu)[\\.\\?!]?$", "Meu pau no teu anel.");

  map
});

