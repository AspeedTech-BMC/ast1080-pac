#[doc = "Register `GPIO918` reader"]
pub type R = crate::R<Gpio918Spec>;
#[doc = "Register `GPIO918` writer"]
pub type W = crate::W<Gpio918Spec>;
#[doc = "Field `GPIO008ReadPrivilegeOfMaster` reader - GPIO008 Read Privilege of Master"]
pub type Gpio008readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO008ReadPrivilegeOfMaster` writer - GPIO008 Read Privilege of Master"]
pub type Gpio008readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO009ReadPrivilegeOfMaster` reader - GPIO009 Read Privilege of Master"]
pub type Gpio009readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO009ReadPrivilegeOfMaster` writer - GPIO009 Read Privilege of Master"]
pub type Gpio009readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO010ReadPrivilegeOfMaster` reader - GPIO010 Read Privilege of Master"]
pub type Gpio010readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO010ReadPrivilegeOfMaster` writer - GPIO010 Read Privilege of Master"]
pub type Gpio010readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO011ReadPrivilegeOfMaster` reader - GPIO011 Read Privilege of Master"]
pub type Gpio011readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO011ReadPrivilegeOfMaster` writer - GPIO011 Read Privilege of Master"]
pub type Gpio011readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO008 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio008read_privilege_of_master(&self) -> Gpio008readPrivilegeOfMasterR {
        Gpio008readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO009 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio009read_privilege_of_master(&self) -> Gpio009readPrivilegeOfMasterR {
        Gpio009readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO010 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio010read_privilege_of_master(&self) -> Gpio010readPrivilegeOfMasterR {
        Gpio010readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO011 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio011read_privilege_of_master(&self) -> Gpio011readPrivilegeOfMasterR {
        Gpio011readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO008 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio008read_privilege_of_master(
        &mut self,
    ) -> Gpio008readPrivilegeOfMasterW<Gpio918Spec> {
        Gpio008readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO009 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio009read_privilege_of_master(
        &mut self,
    ) -> Gpio009readPrivilegeOfMasterW<Gpio918Spec> {
        Gpio009readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO010 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio010read_privilege_of_master(
        &mut self,
    ) -> Gpio010readPrivilegeOfMasterW<Gpio918Spec> {
        Gpio010readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO011 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio011read_privilege_of_master(
        &mut self,
    ) -> Gpio011readPrivilegeOfMasterW<Gpio918Spec> {
        Gpio011readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio918::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio918::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio918Spec;
impl crate::RegisterSpec for Gpio918Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio918::R`](R) reader structure"]
impl crate::Readable for Gpio918Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio918::W`](W) writer structure"]
impl crate::Writable for Gpio918Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO918 to value 0xffff_ffff"]
impl crate::Resettable for Gpio918Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
