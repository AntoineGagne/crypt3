-module(crypt3_nif).

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

load() ->
  erlang:load_nif(filename:join(priv(), "libcrypt3_nif"), none).

%%%===================================================================
%%% Internal functions
%%%===================================================================

not_loaded(Line) ->
  erlang:nif_error({error, {not_loaded, [{module, ?MODULE}, {line, Line}]}}).

priv() ->
  case code:priv_dir(?MODULE) of
    {error, _} ->
      EbinDir = filename:dirname(code:which(?MODULE)),
      AppPath = filename:dirname(EbinDir),
      filename:join(AppPath, "priv");
    Path ->
      Path
  end.
