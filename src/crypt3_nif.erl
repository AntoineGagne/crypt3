-module(crypt3_nif).

%% API
-export([]).

%% Native library support
-export([load/0, not_loaded/1]).
-on_load(load/0).

%%%===================================================================
%%% API
%%%===================================================================

%%%===================================================================
%%% NIF functions
%%%===================================================================

load() ->
  erlang:load_nif(filename:join(priv(), "libcrypt"), none).

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
