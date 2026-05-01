#[doc = "Register `GPIO968` reader"]
pub type R = crate::R<Gpio968Spec>;
#[doc = "Register `GPIO968` writer"]
pub type W = crate::W<Gpio968Spec>;
#[doc = "Field `GPIO088ReadPrivilegeOfMaster` reader - GPIO088 Read Privilege of Master"]
pub type Gpio088readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO088ReadPrivilegeOfMaster` writer - GPIO088 Read Privilege of Master"]
pub type Gpio088readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO089ReadPrivilegeOfMaster` reader - GPIO089 Read Privilege of Master"]
pub type Gpio089readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO089ReadPrivilegeOfMaster` writer - GPIO089 Read Privilege of Master"]
pub type Gpio089readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO090ReadPrivilegeOfMaster` reader - GPIO090 Read Privilege of Master"]
pub type Gpio090readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO090ReadPrivilegeOfMaster` writer - GPIO090 Read Privilege of Master"]
pub type Gpio090readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO091ReadPrivilegeOfMaster` reader - GPIO091 Read Privilege of Master"]
pub type Gpio091readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO091ReadPrivilegeOfMaster` writer - GPIO091 Read Privilege of Master"]
pub type Gpio091readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO088 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio088read_privilege_of_master(&self) -> Gpio088readPrivilegeOfMasterR {
        Gpio088readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO089 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio089read_privilege_of_master(&self) -> Gpio089readPrivilegeOfMasterR {
        Gpio089readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO090 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio090read_privilege_of_master(&self) -> Gpio090readPrivilegeOfMasterR {
        Gpio090readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO091 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio091read_privilege_of_master(&self) -> Gpio091readPrivilegeOfMasterR {
        Gpio091readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO088 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio088read_privilege_of_master(
        &mut self,
    ) -> Gpio088readPrivilegeOfMasterW<Gpio968Spec> {
        Gpio088readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO089 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio089read_privilege_of_master(
        &mut self,
    ) -> Gpio089readPrivilegeOfMasterW<Gpio968Spec> {
        Gpio089readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO090 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio090read_privilege_of_master(
        &mut self,
    ) -> Gpio090readPrivilegeOfMasterW<Gpio968Spec> {
        Gpio090readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO091 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio091read_privilege_of_master(
        &mut self,
    ) -> Gpio091readPrivilegeOfMasterW<Gpio968Spec> {
        Gpio091readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio968::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio968::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio968Spec;
impl crate::RegisterSpec for Gpio968Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio968::R`](R) reader structure"]
impl crate::Readable for Gpio968Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio968::W`](W) writer structure"]
impl crate::Writable for Gpio968Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO968 to value 0xffff_ffff"]
impl crate::Resettable for Gpio968Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
