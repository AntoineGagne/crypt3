-module(crypt3_nif).

-include("crates.hrl").

%% API
-export([encrypt/2]).

%% Native library support
-export([load/0]).

-on_load(load/0).

%%%===================================================================
%%% API
%%%===================================================================

%%%===================================================================
%%% NIF functions
%%%===================================================================

-spec encrypt(Password :: binary(), Hash :: string()) -> ok.
encrypt(_Password, _Hash) ->
    not_loaded(?LINE).

-spec load() -> ok | {error, term()}.
load() ->
    ?load_nif_from_crate(crypt3, ?crate_crypt3_nif, 0).

%%%===================================================================
%%% Internal functions
%%%===================================================================

not_loaded(Line) ->
  erlang:nif_error({error, {not_loaded, [{module, ?MODULE}, {line, Line}]}}).
