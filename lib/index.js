module.exports = ({ wallets, refs, config, client }) => ({
  getOwner: () => client.query("scores", { get_owner: {} }),
  setOwner: (signer = wallets.validator, addr) => client.execute(signer, "scores", { set_owner: { owner: addr } }),
  getScore: (addr, token) => client.query("scores", { get_score: { addr, token } }),
  setScore: (signer = wallets.validator, addr, token, score) => client.execute(signer, "scores", { set_score: { addr, token, score } }),
});
