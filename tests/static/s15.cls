public class Me {
  {
    [SELECT Body__c FROM Banner__c WHERE Language__c = :langCode AND Visible__c = TRUE AND Channel__c INCLUDES (:channel) AND Market__c INCLUDES (:marketCode) ORDER BY Order__c];
  }
}