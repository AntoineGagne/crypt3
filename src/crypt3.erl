-module(crypt3).

%% API
-export([encrypt/2]).

%%%===================================================================
%%% API
%%%===================================================================

-spec encrypt(Password :: binary(), Hash :: binary()) -> ok.
encrypt(Password, Hash) ->
    crypt3_nif:encrypt(Password, Hash).

%%%===================================================================
%%% Internal functions
%%%===================================================================
