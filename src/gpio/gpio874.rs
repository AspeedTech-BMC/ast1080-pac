#[doc = "Register `GPIO874` reader"]
pub type R = crate::R<Gpio874Spec>;
#[doc = "Register `GPIO874` writer"]
pub type W = crate::W<Gpio874Spec>;
#[doc = "Field `GPIO100WrPrivilegeOfMaster` reader - GPIO100 Write Privilege of Master"]
pub type Gpio100wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO100WrPrivilegeOfMaster` writer - GPIO100 Write Privilege of Master"]
pub type Gpio100wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO101WrPrivilegeOfMaster` reader - GPIO101 Write Privilege of Master"]
pub type Gpio101wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO101WrPrivilegeOfMaster` writer - GPIO101 Write Privilege of Master"]
pub type Gpio101wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO102WrPrivilegeOfMaster` reader - GPIO102 Write Privilege of Master"]
pub type Gpio102wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO102WrPrivilegeOfMaster` writer - GPIO102 Write Privilege of Master"]
pub type Gpio102wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO103WrPrivilegeOfMaster` reader - GPIO103 Write Privilege of Master"]
pub type Gpio103wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO103WrPrivilegeOfMaster` writer - GPIO103 Write Privilege of Master"]
pub type Gpio103wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO100 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio100wr_privilege_of_master(&self) -> Gpio100wrPrivilegeOfMasterR {
        Gpio100wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO101 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio101wr_privilege_of_master(&self) -> Gpio101wrPrivilegeOfMasterR {
        Gpio101wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO102 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio102wr_privilege_of_master(&self) -> Gpio102wrPrivilegeOfMasterR {
        Gpio102wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO103 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio103wr_privilege_of_master(&self) -> Gpio103wrPrivilegeOfMasterR {
        Gpio103wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO100 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio100wr_privilege_of_master(&mut self) -> Gpio100wrPrivilegeOfMasterW<Gpio874Spec> {
        Gpio100wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO101 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio101wr_privilege_of_master(&mut self) -> Gpio101wrPrivilegeOfMasterW<Gpio874Spec> {
        Gpio101wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO102 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio102wr_privilege_of_master(&mut self) -> Gpio102wrPrivilegeOfMasterW<Gpio874Spec> {
        Gpio102wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO103 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio103wr_privilege_of_master(&mut self) -> Gpio103wrPrivilegeOfMasterW<Gpio874Spec> {
        Gpio103wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio874::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio874::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio874Spec;
impl crate::RegisterSpec for Gpio874Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio874::R`](R) reader structure"]
impl crate::Readable for Gpio874Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio874::W`](W) writer structure"]
impl crate::Writable for Gpio874Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO874 to value 0xffff_ffff"]
impl crate::Resettable for Gpio874Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
