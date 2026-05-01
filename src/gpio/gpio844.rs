#[doc = "Register `GPIO844` reader"]
pub type R = crate::R<Gpio844Spec>;
#[doc = "Register `GPIO844` writer"]
pub type W = crate::W<Gpio844Spec>;
#[doc = "Field `GPIO052WrPrivilegeOfMaster` reader - GPIO052 Write Privilege of Master"]
pub type Gpio052wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO052WrPrivilegeOfMaster` writer - GPIO052 Write Privilege of Master"]
pub type Gpio052wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO053WrPrivilegeOfMaster` reader - GPIO053 Write Privilege of Master"]
pub type Gpio053wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO053WrPrivilegeOfMaster` writer - GPIO053 Write Privilege of Master"]
pub type Gpio053wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO054WrPrivilegeOfMaster` reader - GPIO054 Write Privilege of Master"]
pub type Gpio054wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO054WrPrivilegeOfMaster` writer - GPIO054 Write Privilege of Master"]
pub type Gpio054wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO055WrPrivilegeOfMaster` reader - GPIO055 Write Privilege of Master"]
pub type Gpio055wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO055WrPrivilegeOfMaster` writer - GPIO055 Write Privilege of Master"]
pub type Gpio055wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO052 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio052wr_privilege_of_master(&self) -> Gpio052wrPrivilegeOfMasterR {
        Gpio052wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO053 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio053wr_privilege_of_master(&self) -> Gpio053wrPrivilegeOfMasterR {
        Gpio053wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO054 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio054wr_privilege_of_master(&self) -> Gpio054wrPrivilegeOfMasterR {
        Gpio054wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO055 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio055wr_privilege_of_master(&self) -> Gpio055wrPrivilegeOfMasterR {
        Gpio055wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO052 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio052wr_privilege_of_master(&mut self) -> Gpio052wrPrivilegeOfMasterW<Gpio844Spec> {
        Gpio052wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO053 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio053wr_privilege_of_master(&mut self) -> Gpio053wrPrivilegeOfMasterW<Gpio844Spec> {
        Gpio053wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO054 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio054wr_privilege_of_master(&mut self) -> Gpio054wrPrivilegeOfMasterW<Gpio844Spec> {
        Gpio054wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO055 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio055wr_privilege_of_master(&mut self) -> Gpio055wrPrivilegeOfMasterW<Gpio844Spec> {
        Gpio055wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio844::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio844::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio844Spec;
impl crate::RegisterSpec for Gpio844Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio844::R`](R) reader structure"]
impl crate::Readable for Gpio844Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio844::W`](W) writer structure"]
impl crate::Writable for Gpio844Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO844 to value 0xffff_ffff"]
impl crate::Resettable for Gpio844Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
