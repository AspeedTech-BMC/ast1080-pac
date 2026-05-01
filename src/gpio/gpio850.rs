#[doc = "Register `GPIO850` reader"]
pub type R = crate::R<Gpio850Spec>;
#[doc = "Register `GPIO850` writer"]
pub type W = crate::W<Gpio850Spec>;
#[doc = "Field `GPIO064WrPrivilegeOfMaster` reader - GPIO064 Write Privilege of Master"]
pub type Gpio064wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO064WrPrivilegeOfMaster` writer - GPIO064 Write Privilege of Master"]
pub type Gpio064wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO065WrPrivilegeOfMaster` reader - GPIO065 Write Privilege of Master"]
pub type Gpio065wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO065WrPrivilegeOfMaster` writer - GPIO065 Write Privilege of Master"]
pub type Gpio065wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO066WrPrivilegeOfMaster` reader - GPIO066 Write Privilege of Master"]
pub type Gpio066wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO066WrPrivilegeOfMaster` writer - GPIO066 Write Privilege of Master"]
pub type Gpio066wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO067WrPrivilegeOfMaster` reader - GPIO067 Write Privilege of Master"]
pub type Gpio067wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO067WrPrivilegeOfMaster` writer - GPIO067 Write Privilege of Master"]
pub type Gpio067wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO064 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio064wr_privilege_of_master(&self) -> Gpio064wrPrivilegeOfMasterR {
        Gpio064wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO065 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio065wr_privilege_of_master(&self) -> Gpio065wrPrivilegeOfMasterR {
        Gpio065wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO066 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio066wr_privilege_of_master(&self) -> Gpio066wrPrivilegeOfMasterR {
        Gpio066wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO067 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio067wr_privilege_of_master(&self) -> Gpio067wrPrivilegeOfMasterR {
        Gpio067wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO064 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio064wr_privilege_of_master(&mut self) -> Gpio064wrPrivilegeOfMasterW<Gpio850Spec> {
        Gpio064wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO065 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio065wr_privilege_of_master(&mut self) -> Gpio065wrPrivilegeOfMasterW<Gpio850Spec> {
        Gpio065wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO066 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio066wr_privilege_of_master(&mut self) -> Gpio066wrPrivilegeOfMasterW<Gpio850Spec> {
        Gpio066wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO067 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio067wr_privilege_of_master(&mut self) -> Gpio067wrPrivilegeOfMasterW<Gpio850Spec> {
        Gpio067wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio850::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio850::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio850Spec;
impl crate::RegisterSpec for Gpio850Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio850::R`](R) reader structure"]
impl crate::Readable for Gpio850Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio850::W`](W) writer structure"]
impl crate::Writable for Gpio850Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO850 to value 0xffff_ffff"]
impl crate::Resettable for Gpio850Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
