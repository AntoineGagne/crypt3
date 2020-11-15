-module(crypt3).

%% API
-export([encrypt/2]).

-type encryption_error() :: crypt3_nif:encryption_error().

-export_type([encryption_error/0]).

%%%===================================================================
%%% API
%%%===================================================================

-spec encrypt(Password :: binary(), Salt :: binary()) ->
    {ok, binary()} | {error, encryption_error()}.
encrypt(Password, Salt) ->
    crypt3_nif:encrypt(Password, Salt).

%%%===================================================================
%%% Internal functions
%%%===================================================================
