-module(crypt3_nif).

-include("crates.hrl").

%% API
-export([encrypt/2]).

%% Native library support
-export([load/0]).

-on_load(load/0).

-type encryption_error() :: {decoding | encoding, Reason :: binary()}.

-export_type([encryption_error/0]).

%%%===================================================================
%%% API
%%%===================================================================

-spec encrypt(Password :: binary(), Salt :: binary()) ->
    {ok, binary()} | {error, encryption_error()}.
encrypt(_Password, _Salt) ->
    not_loaded(?LINE).

%%%===================================================================
%%% NIF functions
%%%===================================================================

-spec load() -> ok | {error, term()}.
load() ->
    ?load_nif_from_crate(crypt3, ?crate_crypt3_nif, 0).

%%%===================================================================
%%% Internal functions
%%%===================================================================

not_loaded(Line) ->
  erlang:nif_error({error, {not_loaded, [{module, ?MODULE}, {line, Line}]}}).
