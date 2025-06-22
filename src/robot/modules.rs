// src/robot/modules.rs - Fonctions utilitaires pour les modules

/// Coût énergétique d'utilisation du module
pub fn module_energy_cost(module: &final_project::RobotModule) -> u32 {
    match module {
        final_project::RobotModule::Deplacement => 1,
        final_project::RobotModule::Communication => 2,
        final_project::RobotModule::AnalyseChimique => 5,
        final_project::RobotModule::ImageHauteResolution => 3,
        final_project::RobotModule::CollecteEnergie => 2,
        final_project::RobotModule::CollecteMineraux => 4,
        final_project::RobotModule::CollecteDonnees => 3,
    }
}

/// Description du module
pub fn module_description(module: &final_project::RobotModule) -> &'static str {
    match module {
        final_project::RobotModule::Deplacement => "Permet au robot de se déplacer sur le terrain",
        final_project::RobotModule::Communication => "Communication avec d'autres robots",
        final_project::RobotModule::AnalyseChimique => "Analyse la composition chimique des échantillons",
        final_project::RobotModule::ImageHauteResolution => "Capture d'images détaillées du terrain",
        final_project::RobotModule::CollecteEnergie => "Collecte des sources d'énergie",
        final_project::RobotModule::CollecteMineraux => "Extraction et collecte de mineraux",
        final_project::RobotModule::CollecteDonnees => "Collecte de données scientifiques",
    }
}

/// Efficacité du module (0.0 à 1.0)
pub fn module_efficiency(module: &final_project::RobotModule) -> f32 {
    match module {
        final_project::RobotModule::Deplacement => 0.9,
        final_project::RobotModule::Communication => 0.95,
        final_project::RobotModule::AnalyseChimique => 0.8,
        final_project::RobotModule::ImageHauteResolution => 0.85,
        final_project::RobotModule::CollecteEnergie => 0.7,
        final_project::RobotModule::CollecteMineraux => 0.75,
        final_project::RobotModule::CollecteDonnees => 0.9,
    }
}

/// Portée d'action du module
pub fn module_range(module: &final_project::RobotModule) -> usize {
    match module {
        final_project::RobotModule::Deplacement => 1,
        final_project::RobotModule::Communication => 10,
        final_project::RobotModule::AnalyseChimique => 1,
        final_project::RobotModule::ImageHauteResolution => 3,
        final_project::RobotModule::CollecteEnergie => 1,
        final_project::RobotModule::CollecteMineraux => 1,
        final_project::RobotModule::CollecteDonnees => 2,
    }
}