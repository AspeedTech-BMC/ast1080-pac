#[doc = "Register `GPIO994` reader"]
pub type R = crate::R<Gpio994Spec>;
#[doc = "Register `GPIO994` writer"]
pub type W = crate::W<Gpio994Spec>;
#[doc = "Field `GPIO132ReadPrivilegeOfMaster` reader - GPIO132 Read Privilege of Master"]
pub type Gpio132readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO132ReadPrivilegeOfMaster` writer - GPIO132 Read Privilege of Master"]
pub type Gpio132readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO133ReadPrivilegeOfMaster` reader - GPIO133 Read Privilege of Master"]
pub type Gpio133readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO133ReadPrivilegeOfMaster` writer - GPIO133 Read Privilege of Master"]
pub type Gpio133readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO134ReadPrivilegeOfMaster` reader - GPIO134 Read Privilege of Master"]
pub type Gpio134readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO134ReadPrivilegeOfMaster` writer - GPIO134 Read Privilege of Master"]
pub type Gpio134readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO135ReadPrivilegeOfMaster` reader - GPIO135 Read Privilege of Master"]
pub type Gpio135readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO135ReadPrivilegeOfMaster` writer - GPIO135 Read Privilege of Master"]
pub type Gpio135readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO132 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio132read_privilege_of_master(&self) -> Gpio132readPrivilegeOfMasterR {
        Gpio132readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO133 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio133read_privilege_of_master(&self) -> Gpio133readPrivilegeOfMasterR {
        Gpio133readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO134 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio134read_privilege_of_master(&self) -> Gpio134readPrivilegeOfMasterR {
        Gpio134readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO135 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio135read_privilege_of_master(&self) -> Gpio135readPrivilegeOfMasterR {
        Gpio135readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO132 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio132read_privilege_of_master(
        &mut self,
    ) -> Gpio132readPrivilegeOfMasterW<Gpio994Spec> {
        Gpio132readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO133 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio133read_privilege_of_master(
        &mut self,
    ) -> Gpio133readPrivilegeOfMasterW<Gpio994Spec> {
        Gpio133readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO134 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio134read_privilege_of_master(
        &mut self,
    ) -> Gpio134readPrivilegeOfMasterW<Gpio994Spec> {
        Gpio134readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO135 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio135read_privilege_of_master(
        &mut self,
    ) -> Gpio135readPrivilegeOfMasterW<Gpio994Spec> {
        Gpio135readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio994::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio994::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio994Spec;
impl crate::RegisterSpec for Gpio994Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio994::R`](R) reader structure"]
impl crate::Readable for Gpio994Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio994::W`](W) writer structure"]
impl crate::Writable for Gpio994Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO994 to value 0xffff_ffff"]
impl crate::Resettable for Gpio994Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
