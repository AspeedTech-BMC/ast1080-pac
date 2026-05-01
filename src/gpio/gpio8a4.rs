#[doc = "Register `GPIO8A4` reader"]
pub type R = crate::R<Gpio8a4Spec>;
#[doc = "Register `GPIO8A4` writer"]
pub type W = crate::W<Gpio8a4Spec>;
#[doc = "Field `GPIO148WrPrivilegeOfMaster` reader - GPIO148 Write Privilege of Master"]
pub type Gpio148wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO148WrPrivilegeOfMaster` writer - GPIO148 Write Privilege of Master"]
pub type Gpio148wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO149WrPrivilegeOfMaster` reader - GPIO149 Write Privilege of Master"]
pub type Gpio149wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO149WrPrivilegeOfMaster` writer - GPIO149 Write Privilege of Master"]
pub type Gpio149wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO150WrPrivilegeOfMaster` reader - GPIO150 Write Privilege of Master"]
pub type Gpio150wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO150WrPrivilegeOfMaster` writer - GPIO150 Write Privilege of Master"]
pub type Gpio150wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO151WrPrivilegeOfMaster` reader - GPIO151 Write Privilege of Master"]
pub type Gpio151wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO151WrPrivilegeOfMaster` writer - GPIO151 Write Privilege of Master"]
pub type Gpio151wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO148 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio148wr_privilege_of_master(&self) -> Gpio148wrPrivilegeOfMasterR {
        Gpio148wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO149 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio149wr_privilege_of_master(&self) -> Gpio149wrPrivilegeOfMasterR {
        Gpio149wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO150 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio150wr_privilege_of_master(&self) -> Gpio150wrPrivilegeOfMasterR {
        Gpio150wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO151 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio151wr_privilege_of_master(&self) -> Gpio151wrPrivilegeOfMasterR {
        Gpio151wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO148 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio148wr_privilege_of_master(&mut self) -> Gpio148wrPrivilegeOfMasterW<Gpio8a4Spec> {
        Gpio148wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO149 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio149wr_privilege_of_master(&mut self) -> Gpio149wrPrivilegeOfMasterW<Gpio8a4Spec> {
        Gpio149wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO150 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio150wr_privilege_of_master(&mut self) -> Gpio150wrPrivilegeOfMasterW<Gpio8a4Spec> {
        Gpio150wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO151 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio151wr_privilege_of_master(&mut self) -> Gpio151wrPrivilegeOfMasterW<Gpio8a4Spec> {
        Gpio151wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8a4Spec;
impl crate::RegisterSpec for Gpio8a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8a4::R`](R) reader structure"]
impl crate::Readable for Gpio8a4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8a4::W`](W) writer structure"]
impl crate::Writable for Gpio8a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8A4 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8a4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
