#[doc = "Register `GPIO870` reader"]
pub type R = crate::R<Gpio870Spec>;
#[doc = "Register `GPIO870` writer"]
pub type W = crate::W<Gpio870Spec>;
#[doc = "Field `GPIO096WrPrivilegeOfMaster` reader - GPIO096 Write Privilege of Master"]
pub type Gpio096wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO096WrPrivilegeOfMaster` writer - GPIO096 Write Privilege of Master"]
pub type Gpio096wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO097WrPrivilegeOfMaster` reader - GPIO097 Write Privilege of Master"]
pub type Gpio097wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO097WrPrivilegeOfMaster` writer - GPIO097 Write Privilege of Master"]
pub type Gpio097wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO098WrPrivilegeOfMaster` reader - GPIO098 Write Privilege of Master"]
pub type Gpio098wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO098WrPrivilegeOfMaster` writer - GPIO098 Write Privilege of Master"]
pub type Gpio098wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO099WrPrivilegeOfMaster` reader - GPIO099 Write Privilege of Master"]
pub type Gpio099wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO099WrPrivilegeOfMaster` writer - GPIO099 Write Privilege of Master"]
pub type Gpio099wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO096 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio096wr_privilege_of_master(&self) -> Gpio096wrPrivilegeOfMasterR {
        Gpio096wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO097 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio097wr_privilege_of_master(&self) -> Gpio097wrPrivilegeOfMasterR {
        Gpio097wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO098 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio098wr_privilege_of_master(&self) -> Gpio098wrPrivilegeOfMasterR {
        Gpio098wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO099 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio099wr_privilege_of_master(&self) -> Gpio099wrPrivilegeOfMasterR {
        Gpio099wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO096 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio096wr_privilege_of_master(&mut self) -> Gpio096wrPrivilegeOfMasterW<Gpio870Spec> {
        Gpio096wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO097 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio097wr_privilege_of_master(&mut self) -> Gpio097wrPrivilegeOfMasterW<Gpio870Spec> {
        Gpio097wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO098 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio098wr_privilege_of_master(&mut self) -> Gpio098wrPrivilegeOfMasterW<Gpio870Spec> {
        Gpio098wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO099 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio099wr_privilege_of_master(&mut self) -> Gpio099wrPrivilegeOfMasterW<Gpio870Spec> {
        Gpio099wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio870::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio870::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio870Spec;
impl crate::RegisterSpec for Gpio870Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio870::R`](R) reader structure"]
impl crate::Readable for Gpio870Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio870::W`](W) writer structure"]
impl crate::Writable for Gpio870Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO870 to value 0xffff_ffff"]
impl crate::Resettable for Gpio870Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
