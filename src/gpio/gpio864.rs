#[doc = "Register `GPIO864` reader"]
pub type R = crate::R<Gpio864Spec>;
#[doc = "Register `GPIO864` writer"]
pub type W = crate::W<Gpio864Spec>;
#[doc = "Field `GPIO084WrPrivilegeOfMaster` reader - GPIO084 Write Privilege of Master"]
pub type Gpio084wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO084WrPrivilegeOfMaster` writer - GPIO084 Write Privilege of Master"]
pub type Gpio084wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO085WrPrivilegeOfMaster` reader - GPIO085 Write Privilege of Master"]
pub type Gpio085wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO085WrPrivilegeOfMaster` writer - GPIO085 Write Privilege of Master"]
pub type Gpio085wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO086WrPrivilegeOfMaster` reader - GPIO086 Write Privilege of Master"]
pub type Gpio086wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO086WrPrivilegeOfMaster` writer - GPIO086 Write Privilege of Master"]
pub type Gpio086wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO087WrPrivilegeOfMaster` reader - GPIO087 Write Privilege of Master"]
pub type Gpio087wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO087WrPrivilegeOfMaster` writer - GPIO087 Write Privilege of Master"]
pub type Gpio087wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO084 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio084wr_privilege_of_master(&self) -> Gpio084wrPrivilegeOfMasterR {
        Gpio084wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO085 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio085wr_privilege_of_master(&self) -> Gpio085wrPrivilegeOfMasterR {
        Gpio085wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO086 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio086wr_privilege_of_master(&self) -> Gpio086wrPrivilegeOfMasterR {
        Gpio086wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO087 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio087wr_privilege_of_master(&self) -> Gpio087wrPrivilegeOfMasterR {
        Gpio087wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO084 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio084wr_privilege_of_master(&mut self) -> Gpio084wrPrivilegeOfMasterW<Gpio864Spec> {
        Gpio084wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO085 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio085wr_privilege_of_master(&mut self) -> Gpio085wrPrivilegeOfMasterW<Gpio864Spec> {
        Gpio085wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO086 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio086wr_privilege_of_master(&mut self) -> Gpio086wrPrivilegeOfMasterW<Gpio864Spec> {
        Gpio086wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO087 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio087wr_privilege_of_master(&mut self) -> Gpio087wrPrivilegeOfMasterW<Gpio864Spec> {
        Gpio087wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio864::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio864::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio864Spec;
impl crate::RegisterSpec for Gpio864Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio864::R`](R) reader structure"]
impl crate::Readable for Gpio864Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio864::W`](W) writer structure"]
impl crate::Writable for Gpio864Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO864 to value 0xffff_ffff"]
impl crate::Resettable for Gpio864Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
