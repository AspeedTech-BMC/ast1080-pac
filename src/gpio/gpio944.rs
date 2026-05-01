#[doc = "Register `GPIO944` reader"]
pub type R = crate::R<Gpio944Spec>;
#[doc = "Register `GPIO944` writer"]
pub type W = crate::W<Gpio944Spec>;
#[doc = "Field `GPIO052ReadPrivilegeOfMaster` reader - GPIO052 Read Privilege of Master"]
pub type Gpio052readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO052ReadPrivilegeOfMaster` writer - GPIO052 Read Privilege of Master"]
pub type Gpio052readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO053ReadPrivilegeOfMaster` reader - GPIO053 Read Privilege of Master"]
pub type Gpio053readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO053ReadPrivilegeOfMaster` writer - GPIO053 Read Privilege of Master"]
pub type Gpio053readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO054ReadPrivilegeOfMaster` reader - GPIO054 Read Privilege of Master"]
pub type Gpio054readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO054ReadPrivilegeOfMaster` writer - GPIO054 Read Privilege of Master"]
pub type Gpio054readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO055ReadPrivilegeOfMaster` reader - GPIO055 Read Privilege of Master"]
pub type Gpio055readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO055ReadPrivilegeOfMaster` writer - GPIO055 Read Privilege of Master"]
pub type Gpio055readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO052 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio052read_privilege_of_master(&self) -> Gpio052readPrivilegeOfMasterR {
        Gpio052readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO053 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio053read_privilege_of_master(&self) -> Gpio053readPrivilegeOfMasterR {
        Gpio053readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO054 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio054read_privilege_of_master(&self) -> Gpio054readPrivilegeOfMasterR {
        Gpio054readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO055 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio055read_privilege_of_master(&self) -> Gpio055readPrivilegeOfMasterR {
        Gpio055readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO052 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio052read_privilege_of_master(
        &mut self,
    ) -> Gpio052readPrivilegeOfMasterW<Gpio944Spec> {
        Gpio052readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO053 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio053read_privilege_of_master(
        &mut self,
    ) -> Gpio053readPrivilegeOfMasterW<Gpio944Spec> {
        Gpio053readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO054 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio054read_privilege_of_master(
        &mut self,
    ) -> Gpio054readPrivilegeOfMasterW<Gpio944Spec> {
        Gpio054readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO055 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio055read_privilege_of_master(
        &mut self,
    ) -> Gpio055readPrivilegeOfMasterW<Gpio944Spec> {
        Gpio055readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio944::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio944::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio944Spec;
impl crate::RegisterSpec for Gpio944Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio944::R`](R) reader structure"]
impl crate::Readable for Gpio944Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio944::W`](W) writer structure"]
impl crate::Writable for Gpio944Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO944 to value 0xffff_ffff"]
impl crate::Resettable for Gpio944Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
