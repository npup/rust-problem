# Försöker...


Detta skall fungera ungefär som en telnetklient utan att (till att börja med) nödvändigtvis implementera _hela_ detta protokoll med div. kontrollkoder osv.

Mitt första mål är att öppna en tcpström mot adress &lt:adress> kontinuerligt och samtidigt

1. läsa från strömmen, och skriva ut (rad för rad) till (t.ex.) stdout
2. läsa användarinput från stdin och skriva denna till den öppnade strömmen

Jag trodde att det skulle gå att dela på strömmen (`tokio.split`) och därefter låta `tokio.select!` (i en evighetsloop) låta det ena eller andra händelsen hanteras (vilken som nu racear "först").

Exemplet (`batmud.bat.org:23`) (och även andra) som jag ansluter till läses relativt snällt in och skrivs ut på skärmen, men  input och skrivning fungerar inte som jag hoppades.

Är det så att "läs-futuren" i `select!` hoggar det hela på något vis?  Om man startar koden med `cargo run` och låter första anslutningen ske (man får se en meny) kan man få NÅGOT slags stdin-input att ske om man hamrar lite på &lt;bokstav>+ENTER (eller bara ENTER).  Men det fungerar som sagt inte alls som jag hoppades.

Tips? Förklarar jag dåligt? :-)  Jag försöker gärna igen i så fall.

