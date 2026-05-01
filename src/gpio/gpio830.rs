#[doc = "Register `GPIO830` reader"]
pub type R = crate::R<Gpio830Spec>;
#[doc = "Register `GPIO830` writer"]
pub type W = crate::W<Gpio830Spec>;
#[doc = "Field `GPIO032WrPrivilegeOfMaster` reader - GPIO032 Write Privilege of Master"]
pub type Gpio032wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO032WrPrivilegeOfMaster` writer - GPIO032 Write Privilege of Master"]
pub type Gpio032wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO033WrPrivilegeOfMaster` reader - GPIO033 Write Privilege of Master"]
pub type Gpio033wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO033WrPrivilegeOfMaster` writer - GPIO033 Write Privilege of Master"]
pub type Gpio033wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO034WrPrivilegeOfMaster` reader - GPIO034 Write Privilege of Master"]
pub type Gpio034wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO034WrPrivilegeOfMaster` writer - GPIO034 Write Privilege of Master"]
pub type Gpio034wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO035WrPrivilegeOfMaster` reader - GPIO035 Write Privilege of Master"]
pub type Gpio035wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO035WrPrivilegeOfMaster` writer - GPIO035 Write Privilege of Master"]
pub type Gpio035wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO032 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio032wr_privilege_of_master(&self) -> Gpio032wrPrivilegeOfMasterR {
        Gpio032wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO033 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio033wr_privilege_of_master(&self) -> Gpio033wrPrivilegeOfMasterR {
        Gpio033wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO034 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio034wr_privilege_of_master(&self) -> Gpio034wrPrivilegeOfMasterR {
        Gpio034wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO035 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio035wr_privilege_of_master(&self) -> Gpio035wrPrivilegeOfMasterR {
        Gpio035wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO032 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio032wr_privilege_of_master(&mut self) -> Gpio032wrPrivilegeOfMasterW<Gpio830Spec> {
        Gpio032wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO033 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio033wr_privilege_of_master(&mut self) -> Gpio033wrPrivilegeOfMasterW<Gpio830Spec> {
        Gpio033wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO034 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio034wr_privilege_of_master(&mut self) -> Gpio034wrPrivilegeOfMasterW<Gpio830Spec> {
        Gpio034wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO035 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio035wr_privilege_of_master(&mut self) -> Gpio035wrPrivilegeOfMasterW<Gpio830Spec> {
        Gpio035wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio830::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio830::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio830Spec;
impl crate::RegisterSpec for Gpio830Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio830::R`](R) reader structure"]
impl crate::Readable for Gpio830Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio830::W`](W) writer structure"]
impl crate::Writable for Gpio830Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO830 to value 0xffff_ffff"]
impl crate::Resettable for Gpio830Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
