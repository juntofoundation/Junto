const { Orchestrator, Config } = require('@holochain/tryorama');
const scenarios = require("../scenarios.js");

const dnaJunto = Config.dna('./dist/junto.dna.json', 'junto');

const mainConfig = Config.gen(
    {
      junto: dnaJunto,  // agent_id="blog", instance_id="blog", dna=dnaBlog
    },
    {
        // specify a bridges
        bridges: [],
        logger: {
            type: 'debug',
            state_dump: false,
            rules: {
                rules: [{ exclude: true, pattern: ".*" }]
            }
        },
        // use a sim2h network
        network: {
            type: 'sim2h',
            sim2h_url: 'wss://sim2h.holochain.org:9000',
        },
    }
);

const orchestrator = new Orchestrator();

String.prototype.format = function() {
    var formatted = this;
    for (var i = 0; i < arguments.length; i++) {
        var regexp = new RegExp('\\{'+i+'\\}', 'gi');
        formatted = formatted.replace(regexp, arguments[i]);
    }
    return formatted;
};

orchestrator.registerScenario('Can post expression and do basic random query', async (s, t) => {
    const {agent1} = await s.players({agent1: mainConfig}, true);
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 1);
    await s.consistency();

    const post_global_expression = await scenarios.postExpression(t, agent1,
        {
            expression: {
                ShortForm: {
                    background: "",
                    body: "This is the first test expression"
                }
            },
            expression_type: "ShortForm"
        },
        ["holochain", "Junto", "social", "holo"],
        [holochain_env.Ok.dna_address]
    );
    await s.consistency();
    const current_date = scenarios.getCurrentTimestamps();
    let current_month = (current_date.month < 10) ? "0"+ current_date.month : current_date.month;
    let current_year = (current_date.year < 10) ? "0"+ current_date.year : current_date.year;
    let current_day = (current_date.day < 10) ? "0" + current_date.day : current_date.day;
    const random_query = await scenarios.queryExpressions(t, agent1, user1.Ok.private_den.address,
        ["social<channel>", "junto<channel>", "holochain<channel>", "holo<channel>", "jdeepee<user>", "shortform<type>", current_year+"<time:y>", current_month+"<time:m>", "0"+current_day+"<time:d>", current_date.hour+"<time:h>"],
        "FilterNew",
        "ExpressionPost",
        "And",
        1,
        "otally random seed",
        false); //0
    t.equal(random_query.Ok.length, 1);
});

const report = orchestrator.run()
console.log(report)