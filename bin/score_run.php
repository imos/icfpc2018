<?php

require_once(dirname(__FILE__) . '/library/config.php');

INFO('Fetching a run for scoring...');

Database::Command('
	UPDATE runs SET
		run_id = (@run_id := run_id),
		run_score_queue = NOW() + INTERVAL 10 SECOND
	WHERE run_score_queue < NOW()
	ORDER BY run_score_queue LIMIT 1');
$run = Database::SelectRow('
	SELECT
		run_id,
		problem_name,
		run_stdout
	FROM runs NATURAL JOIN problems
	WHERE run_id = @run_id');

if (is_null($run)) {
	INFO('Nothing to run.');
	exit(0);
}

INFO("Preparing files...");
file_put_contents(
	'problem', file_get_contents(dirname(__FILE__) . '/../data/problemsF/' . $run['problem_name'] . '_tgt.mdl'));
file_put_contents('assembly', $run['run_stdout']);
file_put_contents('command', 'sim -a assembly -p problem --logtostderr');
file_put_contents('wrapper', '
{ time bash ./command | head -c 30000000 >stdout; } 2>&1 | head -c 1000000 >stderr
');

INFO("Scoring a run (run_id={$run['run_id']})...");
$command = dirname(__FILE__) . '/timeout --timeout=30 bash ./wrapper';
system($command);

$stdout = file_get_contents('stdout');
$energy = NULL;
if (preg_match('%^energy:(\d+)$%Usim', $stdout, $match)) {
	$energy = intval($match[1]);
}

Database::Command('
	UPDATE runs
	SET
		run_score_stdout = {run_score_stdout},
		run_score_stderr = {run_score_stderr},
		run_score = {score},
		run_score_queue = NULL
	WHERE run_id = @run_id', [
	'run_score_stdout' => $stdout,
	'run_score_stderr' => file_get_contents('stderr'),
	'score' => $energy,
]);
