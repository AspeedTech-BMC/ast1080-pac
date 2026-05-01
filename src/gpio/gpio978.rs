#[doc = "Register `GPIO978` reader"]
pub type R = crate::R<Gpio978Spec>;
#[doc = "Register `GPIO978` writer"]
pub type W = crate::W<Gpio978Spec>;
#[doc = "Field `GPIO104ReadPrivilegeOfMaster` reader - GPIO104 Read Privilege of Master"]
pub type Gpio104readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO104ReadPrivilegeOfMaster` writer - GPIO104 Read Privilege of Master"]
pub type Gpio104readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO105ReadPrivilegeOfMaster` reader - GPIO105 Read Privilege of Master"]
pub type Gpio105readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO105ReadPrivilegeOfMaster` writer - GPIO105 Read Privilege of Master"]
pub type Gpio105readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO106ReadPrivilegeOfMaster` reader - GPIO106 Read Privilege of Master"]
pub type Gpio106readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO106ReadPrivilegeOfMaster` writer - GPIO106 Read Privilege of Master"]
pub type Gpio106readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO107ReadPrivilegeOfMaster` reader - GPIO107 Read Privilege of Master"]
pub type Gpio107readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO107ReadPrivilegeOfMaster` writer - GPIO107 Read Privilege of Master"]
pub type Gpio107readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO104 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio104read_privilege_of_master(&self) -> Gpio104readPrivilegeOfMasterR {
        Gpio104readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO105 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio105read_privilege_of_master(&self) -> Gpio105readPrivilegeOfMasterR {
        Gpio105readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO106 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio106read_privilege_of_master(&self) -> Gpio106readPrivilegeOfMasterR {
        Gpio106readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO107 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio107read_privilege_of_master(&self) -> Gpio107readPrivilegeOfMasterR {
        Gpio107readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO104 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio104read_privilege_of_master(
        &mut self,
    ) -> Gpio104readPrivilegeOfMasterW<Gpio978Spec> {
        Gpio104readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO105 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio105read_privilege_of_master(
        &mut self,
    ) -> Gpio105readPrivilegeOfMasterW<Gpio978Spec> {
        Gpio105readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO106 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio106read_privilege_of_master(
        &mut self,
    ) -> Gpio106readPrivilegeOfMasterW<Gpio978Spec> {
        Gpio106readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO107 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio107read_privilege_of_master(
        &mut self,
    ) -> Gpio107readPrivilegeOfMasterW<Gpio978Spec> {
        Gpio107readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio978::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio978::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio978Spec;
impl crate::RegisterSpec for Gpio978Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio978::R`](R) reader structure"]
impl crate::Readable for Gpio978Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio978::W`](W) writer structure"]
impl crate::Writable for Gpio978Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO978 to value 0xffff_ffff"]
impl crate::Resettable for Gpio978Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
