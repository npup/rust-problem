# Rust skojar med mig

Jag sitter i bergen på Kreta med risigt internet och försöker ta upp Rust i syfte att få något att fastna.

Mitt gamla paradnummer i nya språk är att skriva [MUD](https://en.wikipedia.org/wiki/Multi-user_dungeon)-relaterad kod (klient och/eller server) eftersom det brukar innebära kontakt med många intressanta delar av ett programmeringsspråk (I/O, datastrukturer, stränghantering, ev. nätverkande osv.), så det är ämnet även denna gång.
## Vad jag vill

Jag vill skapa något likt vad en telnetklient skulle kunna ge, när man vill ansluta till en s.k. MUD (textabaserat multi-userspel över nätet), t.ex. så här:

    telnet batmud.bat.org 23


Koden jag försöker få till behöver inte (åtminstone inte till att börja med) nödvändigtvis implementera _hela_ telnetprotokollet med div. kontrollkoder osv.

Mitt första mål är att öppna en tcpström mot adress [adress], och sedan, både kontinuerligt och samtidigt

1. läsa från strömmen, och skriva ut (rad för rad) till (t.ex.) stdout
2. läsa användarinput från stdin och skriva denna till den öppnade strömmen för att på så vis skicka kommandon till spelet ("kill goblin")


Så här kan man visst skapa en anslutning:

        const MUD_ADDRESS: &str = "batmud.bat.org:23";
        let server_stream: TcpStream = TcpStream::connect(MUD_ADDRESS).await.unwrap();

Läsa ifrån den går också fint, även om koden är primitiv just nu och inte hanterar några specialsekvenser eller så, utan bara vräker ut allt på skärmen.

Serverexemplet (`batmud.bat.org:23`) (och även andra) som jag ansluter till läses relativt snällt in och skrivs ut på skärmen med kod likt denna:

    reader.read_line(&mut line); // fast await-ad på ena eller andra sättet
    print!("LINE: {}", line);

, men  input och skrivning fungerar inte som jag hoppades.

## Vad jag trodde

Jag trodde att det skulle gå att skapa den ovan nämnda (samtidiga) hanteringen genom att "dela" på strömmen (`tokio.split`, som jag googlade fram på något vis) och därefter genom `tokio.select!` (i en evighetsloop) låta det ena eller andra händelsen hanteras (vilken som nu race-ar "först" hela tiden i denna eviga loop).

## Men..

Om man startar koden i repot med `cargo run` och låter första anslutningen ske (man får se en startmeny från MUDen "Batmud") kan man förvisso få **något** slags stdin-input att ske och skickas om man hamrar lite på [bokstav]+ENTER (eller bara ENTER).  Men det fungerar som sagt inte alls så jämlikt och smidigt som jag hoppades.  
Är det så att "läs-futuren" i `select!` hoggar det hela på något vis?

Är själva idén med split/loop/select mindre bra, och/eller är det något som sticker i ögonen vad gäller själva utförandet?

Jag tar gärna emot tips...  förklarar antagligen dåligt! :-)  Efter ev. följdfråga/or försöker jag gärna igen.

/p
