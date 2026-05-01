#[doc = "Register `GPIO878` reader"]
pub type R = crate::R<Gpio878Spec>;
#[doc = "Register `GPIO878` writer"]
pub type W = crate::W<Gpio878Spec>;
#[doc = "Field `GPIO104WrPrivilegeOfMaster` reader - GPIO104 Write Privilege of Master"]
pub type Gpio104wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO104WrPrivilegeOfMaster` writer - GPIO104 Write Privilege of Master"]
pub type Gpio104wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO105WrPrivilegeOfMaster` reader - GPIO105 Write Privilege of Master"]
pub type Gpio105wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO105WrPrivilegeOfMaster` writer - GPIO105 Write Privilege of Master"]
pub type Gpio105wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO106WrPrivilegeOfMaster` reader - GPIO106 Write Privilege of Master"]
pub type Gpio106wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO106WrPrivilegeOfMaster` writer - GPIO106 Write Privilege of Master"]
pub type Gpio106wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO107WrPrivilegeOfMaster` reader - GPIO107 Write Privilege of Master"]
pub type Gpio107wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO107WrPrivilegeOfMaster` writer - GPIO107 Write Privilege of Master"]
pub type Gpio107wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO104 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio104wr_privilege_of_master(&self) -> Gpio104wrPrivilegeOfMasterR {
        Gpio104wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO105 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio105wr_privilege_of_master(&self) -> Gpio105wrPrivilegeOfMasterR {
        Gpio105wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO106 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio106wr_privilege_of_master(&self) -> Gpio106wrPrivilegeOfMasterR {
        Gpio106wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO107 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio107wr_privilege_of_master(&self) -> Gpio107wrPrivilegeOfMasterR {
        Gpio107wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO104 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio104wr_privilege_of_master(&mut self) -> Gpio104wrPrivilegeOfMasterW<Gpio878Spec> {
        Gpio104wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO105 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio105wr_privilege_of_master(&mut self) -> Gpio105wrPrivilegeOfMasterW<Gpio878Spec> {
        Gpio105wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO106 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio106wr_privilege_of_master(&mut self) -> Gpio106wrPrivilegeOfMasterW<Gpio878Spec> {
        Gpio106wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO107 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio107wr_privilege_of_master(&mut self) -> Gpio107wrPrivilegeOfMasterW<Gpio878Spec> {
        Gpio107wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio878::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio878::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio878Spec;
impl crate::RegisterSpec for Gpio878Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio878::R`](R) reader structure"]
impl crate::Readable for Gpio878Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio878::W`](W) writer structure"]
impl crate::Writable for Gpio878Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO878 to value 0xffff_ffff"]
impl crate::Resettable for Gpio878Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
