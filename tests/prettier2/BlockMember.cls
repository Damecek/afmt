class BlockMember {
  String fieldMemberOne;
  String anotherFieldMember = 'Hello';
  private transient String anotherFieldMemberTwo;
  static string anotherFieldMemberThree;

  String propertyMemberWithGetSet { get; set; }

  void method() {
    System.debug('method');
  }

  class StatementBlockMember {
    {
      System.debug('Statement Block Member');
    }
  }
  class StaticStatementBlockMember {
    static {
      System.debug('Static Statement Block Member');
    }
  }
  interface InnerInterface {
    void interfaceMethod();
  }
}